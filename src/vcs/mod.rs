mod git;

pub trait Vcs {
	/// Returns `true` if there are uncommitted changes
	fn has_changes(&self) -> bool;

	/// Returns `true` if everything is synchronized with the remote repository.
	fn is_synchronized(&self) -> bool;

	fn current_branch(&self) -> String;

	fn symbol(&self) -> &'static str;
}

pub fn repo(dir: &Path) -> Option<Box<Vcs>> {
	let git_repo = git::from_directory(dir);

	match git_repo {
		Some(repo) => Some(box repo as Box<Vcs>),
		None       => None, // or try next vcs
	}
}
