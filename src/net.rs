//! Utilities for determining remote connection information
use std::ffi::CStr;
use std::env;

use libc;

mod native {
	use libc;

	extern {
		pub fn gethostname(name: *mut libc::c_char, len: libc::size_t) -> libc::c_int;
	}
}

pub fn hostname() -> Option<String> {
	let mut buf: Vec<u8> = Vec::with_capacity(255);
	unsafe {
		buf.set_len(255);
		if native::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len() as libc::size_t) == 0 {
			let hostname = CStr::from_ptr(buf.as_ptr() as *const libc::c_char);
			String::from_utf8(hostname.to_bytes().to_vec()).ok() // to_vec copies the buffer, which from_str does not copy
		}
		else {
			None
		}
	}
}

pub fn is_ssh() -> bool {
	match env::var("SSH_CONNECTION") {
		Ok(_)  => true,
		Err(_) => false
	}
}
