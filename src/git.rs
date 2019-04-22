use std::cell::Cell;
use std::fmt;

use failure::{ensure, format_err, Error};
use git2::{
    BranchType, Cred, FetchOptions, Oid, Reference, Remote, RemoteCallbacks, Repository, Status,
    StatusOptions,
};

const BRANCH_REF_PREFIX: &'static str = "refs/heads/";

/// TODO: use Repository::state() to gracefully handle merging, rebasing, etc.

/// Represents the HEAD reference of a Git repository. Depending on the current repository
/// state, this could be the tip of a branch (e.g. master), or a specific commit.
pub enum Head {
    Branch(String),
    Commit(Oid),
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
            Ok(Head::Branch(
                branch_name[BRANCH_REF_PREFIX.len()..].to_string(),
            ))
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
pub fn fetch_current(repo: &Repository) -> Result<(usize, usize), Error> {
    if let Head::Branch(branch_name) = get_head(repo)? {
        let branch = repo.find_branch(&branch_name, BranchType::Local)?;
        assert!(branch.is_head());

        let tracking = match branch.upstream() {
            Ok(upstream) => upstream,
            Err(ref e) if e.code() == git2::ErrorCode::NotFound => return Ok((0, 0)),
            Err(e) => return Err(e.into()),
        };

        let mut remote = match find_remote(repo, tracking.get())? {
            Some(remote) => remote,
            None => return Ok((0, 0)),
        };

        let remote_name = remote.name().ok_or(format_err!("Non-UTF8 remote name"))?;
        let tracking_name = tracking
            .name()?
            .ok_or(format_err!("Non-UTF8 tracking branch"))?;
        ensure!(
            tracking_name.starts_with(remote_name),
            "Cannot determine remote branch name for {}",
            tracking_name
        );

        let remote_branch = &tracking_name[(remote_name.len() + 1)..];

        let tracking_ref = tracking.get();
        let tracking_ref_name = tracking_ref.name().ok_or(format_err!(
            "Reference name for {} is not UTF8",
            tracking_name
        ))?;

        // Use a Cell so we can modify it from the callback
        let upstream_oid = Cell::new(
            tracking_ref
                .target()
                .ok_or(format_err!("{} does not point to a commit", tracking_name))?,
        );

        let mut callbacks = RemoteCallbacks::new();
        callbacks.update_tips(|refname, old, new| {
            if refname == tracking_ref_name {
                assert!(old == upstream_oid.get());
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

        remote.fetch(
            &[remote_branch],
            Some(
                FetchOptions::new()
                    .update_fetchhead(false)
                    .remote_callbacks(callbacks),
            ),
            None,
        )?;

        let local_oid = branch
            .get()
            .target()
            .ok_or(format_err!("Branch does not point to a commit"))?;

        let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid.get())?;
        Ok((ahead, behind))
    } else {
        Ok((0, 0))
    }
}

fn find_remote<'repo>(
    repo: &'repo Repository,
    tracking_reference: &Reference<'repo>,
) -> Result<Option<Remote<'repo>>, Error> {
    let ref_name = tracking_reference
        .name()
        .ok_or(format_err!("Non-UTF8 name for upstream tracking branch"))?;

    for remote_name in repo.remotes()?.iter().flatten() {
        let remote = repo.find_remote(&remote_name)?;

        for refspec in remote.refspecs() {
            if refspec.dst_matches(ref_name) {
                return Ok(Some(remote));
            }
        }
    }

    Ok(None)
}

pub fn is_dirty(repo: &Repository) -> Result<bool, Error> {
    let dirty_status = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_NEW
        | Status::WT_MODIFIED
        | Status::WT_DELETED
        | Status::WT_TYPECHANGE
        | Status::WT_RENAMED
        | Status::CONFLICTED;

    let statuses = repo.statuses(Some(StatusOptions::new().include_untracked(true)))?;
    Ok(statuses
        .iter()
        .any(|se| se.status().intersects(dirty_status)))
}
