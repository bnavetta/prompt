#![allow(non_camel_case_types)]

use super::super::libc::{c_int, c_uint, size_t, uint32_t, uint16_t};

// https://github.com/rust-lang/rust/issues/17056
pub use super::super::libgit2::{git_repository, git_tree, git_submodule_ignore_t, git_strarray, git_diff_file, git_off_t};

// Note for updates: many of the documentation blocks were copied from libgit2 headers
// so I wouldn't have to go back and forth.

pub enum git_status_list {}

/**
* Status flags for a single file.
*
* A combination of these values will be returned to indicate the status of
* a file. Status compares the working directory, the index, and the
* current HEAD of the repository. The `GIT_STATUS_INDEX` set of flags
* represents the status of file in the index relative to the HEAD, and the
* `GIT_STATUS_WT` set of flags represent the status of the file in the
* working directory relative to the index.
*/
bitflags!  {
	#[repr(C)]
	flags git_status_t: uint32_t {
		const GIT_STATUS_CURRENT          = 0,
		const GIT_STATUS_INDEX_NEW        = (1 << 0),
		const GIT_STATUS_INDEX_MODIFIED   = (1 << 1),
		const GIT_STATUS_INDEX_DELETED    = (1 << 2),
		const GIT_STATUS_INDEX_RENAMED    = (1 << 3),
		const GIT_STATUS_INDEX_TYPECHANGE = (1 << 4),
		const GIT_STATUS_WT_NEW           = (1 << 7),
		const GIT_STATUS_WT_MODIFIED      = (1 << 8),
		const GIT_STATUS_WT_DELETED       = (1 << 9),
		const GIT_STATUS_WT_TYPECHANGE    = (1 << 10),
		const GIT_STATUS_WT_RENAMED       = (1 << 11),
		const GIT_STATUS_WT_UNREADABLE    = (1 << 12),
		const GIT_STATUS_IGNORED          = (1 << 14),
	}
}

/**
* Select the files on which to report status.
*
* With `git_status_foreach_ext`, this will control which changes get
* callbacks. With `git_status_list_new`, these will control which
* changes are included in the list.
*
* - GIT_STATUS_SHOW_INDEX_AND_WORKDIR is the default. This roughly
* matches `git status --porcelain` regarding which files are
* included and in what order.
* - GIT_STATUS_SHOW_INDEX_ONLY only gives status based on HEAD to index
* comparison, not looking at working directory changes.
* - GIT_STATUS_SHOW_WORKDIR_ONLY only gives status based on index to
* working directory comparison, not comparing the index to the HEAD.
*/
#[repr(C)]
pub enum git_status_show_t {
	GIT_STATUS_SHOW_INDEX_AND_WORKDIR = 0,
	GIT_STATUS_SHOW_INDEX_ONLY        = 1,
	GIT_STATUS_SHOW_WORKDIR_ONLY      = 2,
}

/**
* Flags to control status callbacks
*
* - GIT_STATUS_OPT_INCLUDE_UNTRACKED says that callbacks should be made
* on untracked files. These will only be made if the workdir files are
* included in the status "show" option.
* - GIT_STATUS_OPT_INCLUDE_IGNORED says that ignored files get callbacks.
* Again, these callbacks will only be made if the workdir files are
* included in the status "show" option.
* - GIT_STATUS_OPT_INCLUDE_UNMODIFIED indicates that callback should be
* made even on unmodified files.
* - GIT_STATUS_OPT_EXCLUDE_SUBMODULES indicates that submodules should be
* skipped. This only applies if there are no pending typechanges to
* the submodule (either from or to another type).
* - GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS indicates that all files in
* untracked directories should be included. Normally if an entire
* directory is new, then just the top-level directory is included (with
* a trailing slash on the entry name). This flag says to include all
* of the individual files in the directory instead.
* - GIT_STATUS_OPT_DISABLE_PATHSPEC_MATCH indicates that the given path
* should be treated as a literal path, and not as a pathspec pattern.
* - GIT_STATUS_OPT_RECURSE_IGNORED_DIRS indicates that the contents of
* ignored directories should be included in the status. This is like
* doing `git ls-files -o -i --exclude-standard` with core git.
* - GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX indicates that rename detection
* should be processed between the head and the index and enables
* the GIT_STATUS_INDEX_RENAMED as a possible status flag.
* - GIT_STATUS_OPT_RENAMES_INDEX_TO_WORKDIR indicates that rename
* detection should be run between the index and the working directory
* and enabled GIT_STATUS_WT_RENAMED as a possible status flag.
* - GIT_STATUS_OPT_SORT_CASE_SENSITIVELY overrides the native case
* sensitivity for the file system and forces the output to be in
* case-sensitive order
* - GIT_STATUS_OPT_SORT_CASE_INSENSITIVELY overrides the native case
* sensitivity for the file system and forces the output to be in
* case-insensitive order
* - GIT_STATUS_OPT_RENAMES_FROM_REWRITES indicates that rename detection
* should include rewritten files
* - GIT_STATUS_OPT_NO_REFRESH bypasses the default status behavior of
* doing a "soft" index reload (i.e. reloading the index data if the
* file on disk has been modified outside libgit2).
* - GIT_STATUS_OPT_UPDATE_INDEX tells libgit2 to refresh the stat cache
* in the index for files that are unchanged but have out of date stat
* information in the index. It will result in less work being done on
* subsequent calls to get status. This is mutually exclusive with the
* NO_REFRESH option.
*
* Calling `git_status_foreach()` is like calling the extended version
* with: GIT_STATUS_OPT_INCLUDE_IGNORED, GIT_STATUS_OPT_INCLUDE_UNTRACKED,
* and GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS. Those options are bundled
* together as `GIT_STATUS_OPT_DEFAULTS` if you want them as a baseline.
*/

#[repr(C)]
pub type git_status_opt_t = u32;

pub const GIT_STATUS_OPT_INCLUDE_UNTRACKED: git_status_opt_t               = (1u << 0) as u32;
pub const GIT_STATUS_OPT_INCLUDE_IGNORED: git_status_opt_t                 = (1u << 1) as u32;
pub const GIT_STATUS_OPT_INCLUDE_UNMODIFIED: git_status_opt_t              = (1u << 2) as u32;
pub const GIT_STATUS_OPT_EXCLUDE_SUBMODULES: git_status_opt_t              = (1u << 3) as u32;
pub const GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS: git_status_opt_t          = (1u << 4) as u32;
pub const GIT_STATUS_OPT_DISABLE_PATHSPEC_MATCH: git_status_opt_t          = (1u << 5) as u32;
pub const GIT_STATUS_OPT_RECURSE_IGNORED_DIRS: git_status_opt_t            = (1u << 6) as u32;
pub const GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX: git_status_opt_t           = (1u << 7) as u32;
pub const GIT_STATUS_OPT_RENAMES_INDEX_TO_WORKDIR: git_status_opt_t        = (1u << 8) as u32;
pub const GIT_STATUS_OPT_SORT_CASE_SENSITIVELY: git_status_opt_t           = (1u << 9) as u32;
pub const GIT_STATUS_OPT_SORT_CASE_INSENSITIVELY: git_status_opt_t         = (1u << 10) as u32;
pub const GIT_STATUS_OPT_RENAMES_FROM_REWRITES: git_status_opt_t           = (1u << 11) as u32;
pub const GIT_STATUS_OPT_NO_REFRESH: git_status_opt_t                      = (1u << 12) as u32;
pub const GIT_STATUS_OPT_UPDATE_INDEX: git_status_opt_t                    = (1u << 13) as u32;
pub const GIT_STATUS_OPT_INCLUDE_UNREADABLE: git_status_opt_t              = (1u << 14) as u32;
pub const GIT_STATUS_OPT_INCLUDE_UNREADABLE_AS_UNTRACKED: git_status_opt_t = (1u << 15) as u32;
pub const GIT_STATUS_OPT_DEFAULTS: git_status_opt_t                        = GIT_STATUS_OPT_INCLUDE_IGNORED
                                                                       | GIT_STATUS_OPT_INCLUDE_UNTRACKED
                                                                       | GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS;

#[repr(C)]
pub struct git_status_options {
	pub version: c_uint,
	pub show: git_status_show_t,
	pub flags: git_status_opt_t,
	pub pathspec: git_strarray
}

#[repr(C)]
pub struct git_status_entry {
	pub status: git_status_t,
	pub head_to_index: *mut git_diff_delta,
	pub index_to_workdir: *mut git_diff_delta,
}

#[repr(C)]
pub struct git_diff_delta {
	pub status: git_delta_t,
	pub flags: uint32_t,
	pub similarity: uint16_t,
	pub nfiles: uint16_t,
	pub old_file: git_diff_file,
	pub new_file: git_diff_file
}

#[repr(C)]
pub enum git_delta_t {
	GIT_DELTA_UNMODIFIED = 0,
	GIT_DELTA_ADDED = 1,
	GIT_DELTA_DELETED = 2,
	GIT_DELTA_MODIFIED = 3,
	GIT_DELTA_RENAMED = 4,
	GIT_DELTA_COPIED = 5,
	GIT_DELTA_IGNORED = 6,
	GIT_DELTA_UNTRACKED = 7,
	GIT_DELTA_TYPECHANGE = 8,
	GIT_DELTA_UNREADABLE = 9,
}

extern {
	pub fn git_status_list_new(list: *mut *mut git_status_list, repo: *mut git_repository, options: *const git_status_options) -> c_int;
	pub fn git_status_byindex(statuslist: *mut git_status_list, idx: size_t) -> *const git_status_entry;
	pub fn git_status_list_entrycount(statuslist: *const git_status_list) -> size_t;
	pub fn git_status_list_free(statuslist: *mut git_status_list);
}
