use std::path::Path;

use git2;
use git2::{Repository, StatusOptions, BranchType};

use super::{Vcs, VcsResult, VcsError};
use super::VcsError::*;

pub struct Git {
	repo: Repository,
}

impl From<git2::Error> for VcsError {
	fn from(e: git2::Error) -> VcsError {
		GitError(e)
	}
}

impl Vcs for Git {
	fn symbol(&self) -> &'static str {
		"±"
	}

	fn is_synchronized(&self) -> VcsResult<bool> {
		let branch_name = try!(self.current_branch());

		let local_branch = try!(self.repo.find_branch(&branch_name, BranchType::Local));
		  // let upstream_branch = try!(local_branch.upstream());
    let upstream_branch = match local_branch.upstream() {
        Ok(branch) => branch,
        Err(_) => return Ok(true), // config lookup fails if branch hasn't ever been pushed (and is therefore synchronized)
    };

		let local_commit = try!(local_branch.get().resolve());
		let remote_commit = try!(upstream_branch.get().resolve());

		Ok(local_commit == remote_commit)
	}

	fn has_changes(&self) -> VcsResult<bool> {
		let mut opts = StatusOptions::new();
		opts.include_ignored(false)
			.include_untracked(true)
			.exclude_submodules(false)
			.pathspec("**/*");

		let statuses = try!(self.repo.statuses(Some(&mut opts)));

		Ok(statuses.iter().filter(|e| e.status() != git2::Status::CURRENT).count() > 0)
	}

	fn current_branch(&self) -> VcsResult<String> {
		let head = try!(self.repo.head());
		match head.name() {
			Some(ref name) => Ok(name[11..].to_string()), // remove the "refs/heads/" part
			None           => Err(Utf8Error),
		}
	}
}

pub fn from_directory(dir: &Path) -> VcsResult<Git> {
	let repo = try!(Repository::discover(dir));
	Ok(Git { repo: repo })
}
