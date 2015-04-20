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
		// let mut config_unstable = try!(self.repo.config());
		// let config = try!(config_unstable.snapshot());
		let branch_name = try!(self.current_branch());

		// let remote_key = format!("branch.{}.remote", branch_name);
		// let remote_name = try!(config.get_str(&remote_key));

		// let remote_branch_name = format!("{}/{}", remote_name, branch_name);

		let local_branch = try!(self.repo.find_branch(&branch_name, BranchType::Local));
		// let remote_branch = try!(self.repo.find_branch(&remote_branch_name, BranchType::Remote));

		let upstream_branch = try!(local_branch.upstream());

		// println!("Upstream Branch: {:?}", upstream_branch.name());
		// println!("Local Branch: {:?}", local_branch.name());
		// println!("Remote Branch: {:?}", remote_branch.name());

		let local_commit = try!(local_branch.get().resolve());
		let remote_commit = try!(upstream_branch.get().resolve());

		Ok(local_commit == remote_commit)

		// let remote_branch_key = format!("branch.{}.merge", branch);
		// let remote_branch = match config.get_str(&remote_branch_key) {
		// 	Ok(branch) => branch[11..], // remove "refs/heads/master",
		// 	Err(_)     => return false
		// };
		//
		// let upstream = format!("refs/remotes/{}/{}", remote_name, remote_branch);
		//
		// let upstream_id = match self.repo.refname_to_id(upstream.as_slice()) {
		// 	Ok(id) => id,
		// 	Err(_) => return false,
		// };
		//
		// let head_id = self.repo.refname_to_id("HEAD").unwrap(); // repo has to have a head
		// let (ahead, behind) = self.repo.graph_ahead_behind(head_id, upstream_id).unwrap();
		// ahead == 0 && behind == 0
	}

	fn has_changes(&self) -> VcsResult<bool> {
		let mut opts = StatusOptions::new();
		opts.include_ignored(false)
			.include_untracked(true)
			.exclude_submodules(false)
			.pathspec("**/*");

		let statuses = try!(self.repo.statuses(Some(&mut opts)));

		Ok(statuses.iter().filter(|e| e.status() != git2::STATUS_CURRENT).count() > 0)
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
