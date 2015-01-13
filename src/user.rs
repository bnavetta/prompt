//! Utilities for retrieving user information
#![allow(unstable)]
use std::ffi;
use std::str;
use libc;
use libc::funcs::posix88::unistd;

/// Determine whether or not the current user is *effectively* root
#[allow(unstable)]
pub fn is_root() -> bool {
	unsafe {
		return unistd::geteuid() == 0;
	}
}

pub fn username() -> Option<String> {
	unsafe {
		let login = unistd::getlogin() as *const libc::c_char;
		if login.is_null() {
			None
		}
		else {
			let login_bytes = ffi::c_str_to_bytes(&login);
			let login_str = str::from_utf8(login_bytes).unwrap();
			Some(String::from_str(login_str))
		}
	}
}
