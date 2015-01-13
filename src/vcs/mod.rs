use std::error::Error;
use std::os;

mod git;

pub trait Vcs {
	/// Create a `Vcs` instance in `dir`.
	/// Returns `None` if initialization failed or the directory was not a
	/// repository / working directory.
	fn from_directory(dir: &Path) -> Option<Self>;

	/// Returns `true` if there are uncommitted changes
	fn has_changes(&self) -> bool;

	/// Returns `true` if everything is synchronized with the remote repository.
	fn is_synchronized(&self) -> bool;

	fn current_branch(&self) -> String;

	fn symbol(&self) -> &'static str;
}

pub fn repo<V>(dir: &Path) -> Option<V> where V: Vcs {
	Vcs::from_directory(dir)
}

pub fn test() {
	let cwd = os::getcwd().unwrap();
	let r: git::Git = repo(&cwd).unwrap();
	println!("Changes? {}", r.has_changes());
	println!("Synchronized? {}", r.is_synchronized());
}
