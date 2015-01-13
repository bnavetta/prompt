#![feature(box_syntax)]
#[allow(unstable)]

extern crate libc;
extern crate git2;

use std::os;
use std::time::duration;

pub mod mux;
pub mod net;
pub mod user;
pub mod vcs;

pub fn cwd() -> Path {
	#![allow(unstable)]
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

pub fn human_time(duration: duration::Duration) -> String {
	#![allow(unstable)]
	let mut result = String::new();

	let days = duration.num_days();
	let hours = duration.num_hours() % 24;
	let minutes = duration.num_minutes() % 60;
	let seconds = duration.num_seconds() % 60;

	// I feel like there has to be a better way than to_string().as_slice() - can I just add the actual String?
	if days > 0
	{
		result.push_str(days.to_string().as_slice());
		result.push_str("d ");
	}

	if hours > 0
	{
		result.push_str(hours.to_string().as_slice());
		result.push_str("h ");
	}

	if minutes > 0
	{
		result.push_str(minutes.to_string().as_slice());
		result.push_str("m ");
	}

	result.push_str(seconds.to_string().as_slice());
	result.push_str("s");

	result
}
