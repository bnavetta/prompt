//! Utilities for retrieving user information
use std::env;

use nix::unistd;

/// Determine whether or not the current user is *effectively* root
pub fn is_root() -> bool {
	unistd::geteuid().is_root()
}

pub fn username() -> Option<String> {
	// According to http://linux.die.net/man/3/getlogin, it is better to use $LOGNAME than to
	// use getlogin/getlogin_r/cuserid
	env::var("LOGNAME").ok()
}
