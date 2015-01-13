#[allow(unstable)]
extern crate libc;
extern crate git2;

use std::os;

pub mod mux;
pub mod net;
pub mod user;
pub mod vcs;

pub fn cwd() -> Path {
	// TODO: path shortening?

	let home_dir = os::homedir().unwrap();
	let current_dir = os::getcwd().unwrap();

	if home_dir.is_ancestor_of(&current_dir) {
		Path::new("~").join(current_dir.path_relative_from(&home_dir).unwrap())
	}
	else {
		current_dir
	}
}
