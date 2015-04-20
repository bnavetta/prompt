#![feature(std_misc)]
#![feature(path_relative_from)]

extern crate libc;
extern crate git2;

use std::env;
use std::path::Path;
use std::time::duration;

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

pub fn human_duration(duration: duration::Duration) -> String {
	let mut result = String::new();

	let days = duration.num_days();
	let hours = duration.num_hours() % 24;
	let minutes = duration.num_minutes() % 60;
	let seconds = duration.num_seconds() % 60;

	if days > 0 {
		result.push_str(&days.to_string());
		result.push_str("d ");
	}

	if hours > 0 {
		result.push_str(&hours.to_string());
		result.push_str("h ");
	}

	if minutes > 0 {
		result.push_str(&minutes.to_string());
		result.push_str("m ");
	}

	result.push_str(&seconds.to_string());
	result.push_str("s");

	result
}
