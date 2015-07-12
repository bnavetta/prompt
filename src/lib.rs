#![feature(std_misc)]
#![feature(path_relative_from)]

extern crate libc;
extern crate git2;

use std::env;
use std::path::Path;

pub mod mux;
pub mod net;
pub mod user;
pub mod vcs;

pub fn cwd() -> String {
	// TODO: path shortening?

	let home_dir = env::home_dir().unwrap();
	let current_dir = env::current_dir().unwrap();

	let cwd = if current_dir.starts_with(&home_dir) {
		Path::new("~").join(current_dir.relative_from(&home_dir).unwrap())
	}
	else {
		current_dir
	};

	format!("{}", cwd.display())
}
