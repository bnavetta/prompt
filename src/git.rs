use std::cell::Cell;
use std::fmt;

use failure::{Error, ResultExt, format_err, ensure};
use git2::{Repository, ReferenceType, Oid, BranchType, Reference, Remote, FetchOptions, RemoteCallbacks, Cred};

const BRANCH_REF_PREFIX: &'static str = "refs/heads/";

/// TODO: use Repository::state() to gracefully handle merging, rebasing, etc.

/// Represents the HEAD reference of a Git repository. Depending on the current repository
/// state, this could be the tip of a branch (e.g. master), or a specific commit.
pub enum Head {
    Branch(String),
    Commit(Oid)
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Head::Branch(ref branch) => branch.fmt(f),
            &Head::Commit(ref commit) => write!(f, "{:.8}", commit),
        }
    }
}

pub fn get_head(repo: &Repository) -> Result<Head, Error> {
    let head = repo.head()?;

    if head.is_branch() {
        let branch_name = head.name().ok_or(format_err!("Missing HEAD name"))?;
        if branch_name.starts_with(BRANCH_REF_PREFIX) {
            Ok(Head::Branch(branch_name[BRANCH_REF_PREFIX.len()..].to_string()))
        } else {
            Err(format_err!("Invalid branch reference {}", branch_name))
        }
    } else {
        let commit_oid = head.target().ok_or(format_err!("HEAD missing a target"))?;
        Ok(Head::Commit(commit_oid))
    }

    // TODO: can we ever have a non-branch HEAD that isn't a direct reference?
}

/// fetch_current does a fetch for the upstream of the currently-active branch. If HEAD
/// does not point to a branch, nothing is done.
pub fn fetch_current(repo: &Repository) -> Result<(), Error> {
    if let Head::Branch(branch_name) = get_head(repo)? {
        let branch = repo.find_branch(&branch_name, BranchType::Local)?;
        assert!(branch.is_head());

        let upstream = branch.upstream()?;
        let mut remote = find_remote(repo, upstream.get())?;

        let remote_name = remote.name().ok_or(format_err!("Non-UTF8 remote name"))?;
        let upstream_name = upstream.name()?.ok_or(format_err!("Non-UTF8 tracking branch"))?;
        ensure!(upstream_name.starts_with(remote_name), "Cannot determine upstream branch name");
        let upstream_branch_name = &upstream_name[(remote_name.len() + 1)..];

        let tip_ref_name = upstream.get().name().ok_or(format_err!("Non-UTF8 upstream reference"))?;
        // Use a Cell so we can modify it from the callback
        let upstream_oid = Cell::new(upstream.get().target().ok_or(format_err!("No upstream commit recorded"))?);

        let mut callbacks = RemoteCallbacks::new();
        callbacks.update_tips(|refname, old, new| {
            if refname == tip_ref_name {
                // assert!(old == upstream_oid);
                upstream_oid.set(new);
            }
            true
        });
        callbacks.credentials(|url, username, _allowed_types| {
            let config = repo.config()?;

            Cred::credential_helper(&config, url, username).or_else(|_| {
                if let Some(username) = username {
                    Cred::ssh_key_from_agent(username)
                } else {
                    Err(git2::Error::from_str("No username for querying SSH agent"))
                }
            })
        });
        let mut fetch_options = FetchOptions::new();
        fetch_options.update_fetchhead(false).remote_callbacks(callbacks);
        remote.fetch(&[upstream_branch_name], Some(&mut fetch_options), None)?;

        let current_oid = branch.get().target().ok_or(format_err!("Could not find branch tip"))?;
        let (ahead, behind) = repo.graph_ahead_behind(current_oid, upstream_oid.get())?;
        println!("{} commits ahead, {} commits behind", ahead, behind);

        Ok(())
    } else {
        Ok(())
    }
}

fn find_remote<'repo>(repo: &'repo Repository, tracking_reference: &Reference<'repo>) -> Result<Remote<'repo>, Error> {
    let ref_name = tracking_reference.name().ok_or(format_err!("Non-UTF8 name for upstream tracking branch"))?;

    for remote_name in repo.remotes()?.iter().flatten() {
        let remote = repo.find_remote(&remote_name)?;

        for refspec in remote.refspecs() {
            if refspec.dst_matches(ref_name) {
                return Ok(remote);
            }
        }
    }

    Err(format_err!("Unable to find remote"))
}