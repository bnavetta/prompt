#![allow(non_camel_case_types)]

use std::c_str::CString;
use std::kinds::marker;
use super::libc::{c_char, c_uint, size_t};
use super::git2::{Repository, Error, Oid};

pub mod raw;

pub enum StatusShow {
	ShowIndexAndWorkdir = raw::GIT_STATUS_SHOW_INDEX_AND_WORKDIR as int,
	ShowIndexOnly       = raw::GIT_STATUS_SHOW_INDEX_ONLY as int,
	ShowWorkdirOnly     = raw::GIT_STATUS_SHOW_WORKDIR_ONLY as int,
}

impl StatusShow {
	pub fn from_raw(raw: raw::git_status_show_t) -> StatusShow {
		match raw {
			raw::GIT_STATUS_SHOW_INDEX_AND_WORKDIR => ShowIndexAndWorkdir,
			raw::GIT_STATUS_SHOW_INDEX_ONLY        => ShowIndexOnly,
			raw::GIT_STATUS_SHOW_WORKDIR_ONLY      => ShowWorkdirOnly,
		}
	}

	pub fn raw(self) -> raw::git_status_show_t {
		match self {
			ShowIndexAndWorkdir => raw::GIT_STATUS_SHOW_INDEX_AND_WORKDIR,
			ShowIndexOnly       => raw::GIT_STATUS_SHOW_INDEX_ONLY,
			ShowWorkdirOnly     => raw::GIT_STATUS_SHOW_WORKDIR_ONLY,
		}
	}
}

bitflags! {
	flags StatusOpt: u32 {
		const OPT_INCLUDE_UNTRACKED               = raw::GIT_STATUS_OPT_INCLUDE_UNTRACKED,
		const OPT_INCLUDE_IGNORED                 = raw::GIT_STATUS_OPT_INCLUDE_IGNORED,
		const OPT_INCLUDE_UNMODIFIED              = raw::GIT_STATUS_OPT_INCLUDE_UNMODIFIED,
		const OPT_EXCLUDE_SUBMODULES              = raw::GIT_STATUS_OPT_EXCLUDE_SUBMODULES,
		const OPT_RECURSE_UNTRACKED_DIRS          = raw::GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS,
		const OPT_DISABLE_PATHSPEC_MATCH          = raw::GIT_STATUS_OPT_DISABLE_PATHSPEC_MATCH,
		const OPT_RECURSE_IGNORED_DIRS            = raw::GIT_STATUS_OPT_RECURSE_IGNORED_DIRS,
		const OPT_RENAMES_HEAD_TO_INDEX           = raw::GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX,
		const OPT_RENAMES_INDEX_TO_WORKDIR        = raw::GIT_STATUS_OPT_RENAMES_INDEX_TO_WORKDIR,
		const OPT_SORT_CASE_INSENSITIVELY         = raw::GIT_STATUS_OPT_SORT_CASE_INSENSITIVELY,
		const OPT_SORT_CASE_SENSITIVELY           = raw::GIT_STATUS_OPT_SORT_CASE_SENSITIVELY,
		const OPT_RENAMES_FROM_REWRITES           = raw::GIT_STATUS_OPT_RENAMES_FROM_REWRITES,
		const OPT_NO_REFRESH                      = raw::GIT_STATUS_OPT_NO_REFRESH,
		const OPT_UPDATE_INDEX                    = raw::GIT_STATUS_OPT_UPDATE_INDEX,
		const OPT_INCLUDE_UNREADABLE              = raw::GIT_STATUS_OPT_INCLUDE_UNREADABLE,
		const OPT_INCLUDE_UNREADABLE_AS_UNTRACKED = raw::GIT_STATUS_OPT_INCLUDE_UNREADABLE_AS_UNTRACKED,
		const OPT_DEFAULTS                        = raw::GIT_STATUS_OPT_DEFAULTS,
	}
}

impl StatusOpt {
	pub fn from_raw(raw: c_uint) -> StatusOpt {
		StatusOpt::from_bits_truncate(raw)
	}

	pub fn raw(&self) -> c_uint {
		self.bits as c_uint
	}
}

pub struct StatusOptions<'a> {
	pub show: StatusShow,
	pub flags: StatusOpt,
	pub pathspec: Vec<&'a str>,
}

impl<'a> StatusOptions<'a> {
	pub fn defaults() -> StatusOptions<'a> {
		StatusOptions {
			show: ShowIndexAndWorkdir,
			flags: StatusOpt::empty(),
			pathspec: Vec::new(),
		}
	}

	pub fn raw(&self) -> raw::git_status_options {
		let pathspec = if self.pathspec.is_empty() {
			raw::git_strarray {
				strings: 0 as *mut _,
				count: 0 as size_t,
			}
		} else {
			let arr = self.pathspec.iter().map(|p| p.to_c_str()).collect::<Vec<CString>>();
			let strarray = arr.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();
			raw::git_strarray {
				strings: strarray.as_ptr() as *mut _,
				count: strarray.len() as size_t,
			}
		};

		raw::git_status_options {
			version: 1,
			show: self.show.raw(),
			flags: self.flags.raw() as raw::git_status_opt_t,
			pathspec: pathspec
		}
	}
}

pub struct StatusList {
	raw: *mut raw::git_status_list,
	marker1: marker::NoSend,
	marker2: marker::NoSync,
}

impl StatusList {
	pub fn new(repo: &Repository, opts: StatusOptions) -> Result<StatusList, Error> {
		let mut raw = 0 as *mut raw::git_status_list;
		unsafe {
			let raw_opts = opts.raw();
			let rc = raw::git_status_list_new(&mut raw, repo.raw(), &raw_opts as *const raw::git_status_options);
			if rc < 0 {
				return Err(Error::last_error().unwrap_or_else(|| {
					Error::from_str("an unknown error occurred")
				}));
			}
			else {
				Ok(StatusList::from_raw(raw))
			}
		}
	}

	pub unsafe fn from_raw(raw: *mut raw::git_status_list) -> StatusList {
		StatusList {
			raw: raw,
			marker1: marker::NoSend,
			marker2: marker::NoSync,
		}
	}

	pub fn raw(&self) -> *mut raw::git_status_list {
		self.raw
	}

	pub fn entry_count(&self) -> uint {
		unsafe {
			raw::git_status_list_entrycount(self.raw as *const raw::git_status_list) as uint
		}
	}

	pub fn by_index(&self, idx: uint) -> StatusEntry {
		unsafe {
			StatusEntry::from_raw(raw::git_status_byindex(self.raw, idx as u64))
		}
	}

	pub fn iter<'a>(&'a self) -> StatusEntryIterator<'a> {
		StatusEntryIterator {
			list: self,
			index: 0
		}
	}
}

#[unsafe_destructor]
impl Drop for StatusList {
	fn drop(&mut self) {
		unsafe {
			raw::git_status_list_free(self.raw);
		}
	}
}

pub struct StatusEntryIterator<'a> {
	list: &'a StatusList,
	index: uint,
}

impl<'a> Iterator<StatusEntry> for StatusEntryIterator<'a> {
	fn next(&mut self) -> Option<StatusEntry> {
		if self.index < self.list.entry_count() {
			let result = Some(self.list.by_index(self.index));
			self.index += 1;
			result
		}
		else
		{
			None
		}
	}
}

bitflags! {
	flags Status: uint {
		const Current           = 0,
		const IndexNew          = (1u << 0),
		const IndexModified     = (1u << 0),
		const IndexDeleted      = (1u << 0),
		const IndexRenamed      = (1u << 0),
		const IndexTypechange   = (1u << 0),
		const WorkdirNew        = (1u << 0),
		const WorkdirModified   = (1u << 0),
		const WorkdirDeleted    = (1u << 0),
		const WorkdirTypechange = (1u << 0),
		const WorkdirRenamed    = (1u << 0),
		const WorkdirUnreadable = (1u << 0),
		const Ignored           = (1u << 0),
	}
}

impl Status {
	fn from_raw(raw: raw::git_status_t) -> Status {
		Status::from_bits(raw.bits() as uint).unwrap()
	}
}

pub struct StatusEntry {
	raw: *const raw::git_status_entry,
	marker1: marker::NoSend,
	marker2: marker::NoSync,
}

impl StatusEntry {
	pub unsafe fn from_raw(raw: *const raw::git_status_entry) -> StatusEntry {
		StatusEntry {
			raw: raw,
			marker1: marker::NoSend,
			marker2: marker::NoSync,
		}
	}

	pub fn status(&self) -> Status {
		unsafe {
			Status::from_raw((*self.raw).status)
		}
	}

	pub fn head_to_index(&self) -> DiffDelta {
		unsafe {
			DiffDelta::from_raw((*self.raw).head_to_index)
		}
	}

	pub fn index_to_workdir(&self) -> DiffDelta {
		unsafe {
			DiffDelta::from_raw((*self.raw).index_to_workdir)
		}
	}
}

pub enum Delta {
	Unmodified,
	Added,
	Deleted,
	Modified,
	Renamed,
	Copied,
	DeltaIgnored,
	Untracked,
	Typechange,
	Unreadable,
}

impl Delta {
	pub fn from_raw(raw: raw::git_delta_t) -> Delta {
		match raw {
			raw::GIT_DELTA_UNMODIFIED => Unmodified,
			raw::GIT_DELTA_ADDED      => Added,
			raw::GIT_DELTA_DELETED    => Deleted,
			raw::GIT_DELTA_MODIFIED   => Modified,
			raw::GIT_DELTA_RENAMED    => Renamed,
			raw::GIT_DELTA_COPIED     => Copied,
			raw::GIT_DELTA_IGNORED    => DeltaIgnored,
			raw::GIT_DELTA_UNTRACKED  => Untracked,
			raw::GIT_DELTA_TYPECHANGE => Typechange,
			raw::GIT_DELTA_UNREADABLE => Unreadable,
		}
	}
}

pub struct DiffFile {
	raw: *mut raw::git_diff_file,
	marker1: marker::NoSend,
	marker2: marker::NoSync,
}

impl DiffFile {
	pub unsafe fn from_raw(raw: *mut raw::git_diff_file) -> DiffFile {
		return DiffFile {
			raw: raw,
			marker1: marker::NoSend,
			marker2: marker::NoSync,
		}
	}

	pub fn id(&self) -> Oid {
		unsafe {
			let id = (*self.raw).id;
			Oid::from_raw(&id)
		}
	}

	pub fn path(&self) -> String {
		let c_path = unsafe {
			CString::new((*self.raw).path, false)
		};
		String::from_str(c_path.as_str().unwrap())
	}

	pub fn size(&self) -> u64 {
		unsafe {
			(*self.raw).size as u64
		}
	}
}

pub struct DiffDelta {
	raw: *mut raw::git_diff_delta,
	marker1: marker::NoSend,
	marker2: marker::NoSync,
}

impl DiffDelta {
	pub unsafe fn from_raw(raw: *mut raw::git_diff_delta) -> DiffDelta {
		DiffDelta {
			raw: raw,
			marker1: marker::NoSend,
			marker2: marker::NoSync,
		}
	}

	pub fn status(&self) -> Delta {
		unsafe {
			Delta::from_raw((*self.raw).status)
		}
	}

	pub fn similiarity(&self) -> u16 {
		unsafe {
			(*self.raw).similarity as u16
		}
	}

	pub fn nfiles(&self) -> uint {
		unsafe {
			(*self.raw).nfiles as uint
		}
	}

	pub fn old_file(&self) -> DiffFile {
		unsafe {
			let mut old_file = (*self.raw).old_file;
			DiffFile::from_raw(&mut old_file as *mut raw::git_diff_file)
		}
	}

	pub fn new_file(&self) -> DiffFile {
		unsafe {
			let mut new_file = (*self.raw).new_file;
			DiffFile::from_raw(&mut new_file as *mut raw::git_diff_file)
		}
	}
}
