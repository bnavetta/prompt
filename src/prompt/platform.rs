extern crate libc;

use std::mem;
use std::str;
use std::string;

#[repr(C)]
struct c_passwd {
	pw_name: *const libc::c_char,
	pw_uid: libc::uid_t,
	pw_gid: libc::gid_t,
	pw_dir: *const libc::c_char,
	pw_shell: *const libc::c_char
}

// See https://github.com/uutils/coreutils/blob/master/src/whoami/whoami.rs
// and https://github.com/uutils/coreutils/blob/master/src/hostname/hostname.rs
extern {
	fn geteuid() -> libc::uid_t;
	fn getpwuid(uid: libc::uid_t) -> *const c_passwd;

	fn gethostname(name: *mut libc::c_char, len: libc::size_t) -> libc::c_int;
}

pub fn username() -> String {
	unsafe {
		let passwd = getpwuid(geteuid());

		string::raw::from_buf((*passwd).pw_name as *const u8)
	}
}

pub fn hostname() -> String {
	let mut buf: Vec<u8> = Vec::with_capacity(100);

	let rc = unsafe {
		gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len() as libc::size_t)
	};

	if rc != 0 {
		fail!("Error determining hostname");
	}

	let last_char = buf.iter().position(|byte| *byte == 0).unwrap_or(buf.len());
	unsafe {
		// str::raw::from_utf8(buf.slice_to(last_char))
		string::raw::from_utf8(buf)
	}

}

#[cfg(test)]
mod tests {
	use super::{username, hostname};

	#[test]
	fn test_username() {
		assert_eq!(username().as_slice(), env!("USER"));
	}

	#[test]
	fn test_hostname() {
		// assert_eq!(hostname().as_slice(), "spock");
	}
}
