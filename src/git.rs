use std::fmt;

use failure::{Error, ResultExt, format_err};
use git2::{Repository, ReferenceType, Oid};

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

    println!("Head name is {:?}", head.name());
    println!("Branch? {}", head.is_branch());

    match head.kind() {
        Some(ReferenceType::Oid) => head.target().ok_or(format_err!("HEAD missing a target")).map(Head::Commit),
        Some(ReferenceType::Symbolic) => head.symbolic_target().ok_or(format_err!("HEAD missing a symbolic target")).map(|b| Head::Branch(b.to_string())),
        None => Err(format_err!("Unable to determine type of HEAD reference"))
    }
}

/// fetch_current does a fetch for the upstream of the currently-active branch
pub fn fetch_current(repo: &Repository) -> Result<(), Error> {
    let head = repo.head()?;

    Ok(())
}