use std::fmt;

use failure::{Error, ResultExt, format_err};
use git2::{Repository, ReferenceType, Oid};

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
            &Head::Commit(ref commit) => commit.fmt(f),
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

    // TODO: can we ever have a non-branch HEAD that isn't an Oid reference?
}

/// fetch_current does a fetch for the upstream of the currently-active branch
pub fn fetch_current(repo: &Repository) -> Result<(), Error> {
    let head = repo.head()?;

    Ok(())
}