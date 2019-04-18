mod git;

use std::error::Error;
use std::fmt;
use std::path::Path;

use git2;

use self::VcsError::*;

pub trait Vcs {
	/// Returns `true` if there are uncommitted changes
	fn has_changes(&self) -> VcsResult<bool>;

	/// Returns `true` if everything is synchronized with the remote repository.
	fn is_synchronized(&self) -> VcsResult<bool>;

	fn current_branch(&self) -> VcsResult<String>;

	fn symbol(&self) -> &'static str;
}

#[derive(Debug)]
pub enum VcsError {
	GitError(git2::Error),
	Utf8Error,
}

impl Error for VcsError {
	fn description(&self) -> &str {
		match *self {
			GitError(ref e) => e.message(),
			Utf8Error       => "Invalid UTF-8 text",
		}
	}

	fn cause(&self) -> Option<&Error> {
		match *self {
			GitError(ref e) => Some(e),
			_               => None
		}
	}
}

impl fmt::Display for VcsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let error_type = match *self {
			GitError(_) => "GitError",
			Utf8Error   => "Utf8Error",
		};

		write!(f, "VcsError.{}[{}]", error_type, self.description())
	}
}

pub type VcsResult<T> = Result<T, VcsError>;

pub fn repo(dir: &Path) -> VcsResult<Box<Vcs>> {
	let git_repo = git::from_directory(dir).map(|git| Box::new(git) as Box<Vcs>);

	git_repo // git_repo.or(..) once more are supported
}
