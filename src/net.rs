//! Utilities for determining remote connection information
use std::ffi;
use std::os;
use std::str;

use libc;

mod native {
	use libc;

	extern {
		pub fn gethostname(name: *mut libc::c_char, len: libc::size_t) -> libc::c_int;
	}
}

pub fn hostname() -> String {
	let mut buf = [0 as libc::c_char; 255];
	unsafe {
		if native::gethostname(buf.as_mut_ptr(), buf.len() as libc::size_t) == 0 {
			String::from_str(
				str::from_utf8(ffi::c_str_to_bytes(&buf.as_ptr())).unwrap()
			)
		}
		else {
			panic!("gethostname() failed: {}", os::error_string(os::errno()));
		}
	}
}

pub fn is_ssh() -> bool {
	match os::getenv("SSH_CONNECTION") {
		Some(_) => true,
		None    => false
	}
}
