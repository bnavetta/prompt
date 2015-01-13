#![allow(unstable)]

use git2::{self, Repository, StatusOptions};

pub struct Git {
	repo: Repository,
}

impl super::Vcs for Git {

	fn from_directory(dir: &Path) -> Option<Git> {
		match Repository::discover(dir) {
			Ok(repo) => Some(Git { repo: repo }),
			Err(_)   => None,
		}
	}

	fn symbol(&self) -> &'static str {
		"±"
	}

	fn is_synchronized(&self) -> bool {
		let config = match self.repo.config() {
			Ok(config) => config,
			Err(_)     => return false,
		};

		let branch = self.current_branch();

		let remote_name = match config.get_str(format!("branch.{}.remote", branch).as_slice()) {
			Ok(name) => name,
			Err(_)   => return false,
		};

		let remote_branch = match config.get_str(format!("branch.{}.merge", branch).as_slice()) {
			Ok(branch) => branch.slice_from(11), // remove "refs/heads/master",
			Err(_)     => return false
		};

		let upstream = format!("refs/remotes/{}/{}", remote_name, remote_branch);

		let upstream_id = match self.repo.refname_to_id(upstream.as_slice()) {
			Ok(id) => id,
			Err(_) => return false,
		};

		let head_id = self.repo.refname_to_id("HEAD").unwrap(); // repo has to have a head
		let (ahead, behind) = self.repo.graph_ahead_behind(head_id, upstream_id).unwrap();
		ahead == 0 && behind == 0
	}

	fn has_changes(&self) -> bool {
		let mut opts = StatusOptions::new();
		opts.include_ignored(false)
			.include_untracked(true)
			.exclude_submodules(false)
			.pathspec("**/*");

		let statuses = match self.repo.statuses(Some(&mut opts)) {
			Ok(statuses) => statuses,
			Err(_)       => return false, // TODO: or true?
		};

		statuses.iter().filter(|e| e.status() != git2::STATUS_CURRENT).count() > 0
	}

	fn current_branch(&self) -> String {
		let head = self.repo.head().unwrap();
		let branch = head.name().unwrap().slice_from(11); // remove the "refs/heads/" part
		branch.to_string()
	}
}
