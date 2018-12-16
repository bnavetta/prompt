//! Utilities for determining remote connection information
use std::env;

use nix::unistd;

pub fn hostname() -> Option<String> {
	let mut buf = [0u8; 256];

	let hostname_cstr = match unistd::gethostname(&mut buf) {
		Ok(cstr) => cstr,
		_ => return None
	};

	match hostname_cstr.to_str() {
		Ok(s) => Some(s.to_string()),
		_ => None 
	}
}

pub fn is_ssh() -> bool {
	match env::var("SSH_CONNECTION") {
		Ok(_)  => true,
		Err(_) => false
	}
}
