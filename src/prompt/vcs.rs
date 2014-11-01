use std::os;

use super::git2::{Repository, Error};
use super::git::{StatusList};

pub struct VCSInfo
{
	pub branch: String,
	pub clean: bool
}

pub fn vcs_info() -> Result<VCSInfo, Error>
{
	let path = os::getcwd();

	let repo = try!(Repository::open(&path));
	let status_list = try!(StatusList::new(&repo));

	for entry in status_list.iter() {
		println!("{} -> {}", entry.index_to_workdir().old_file().path(), entry.index_to_workdir().new_file().path());
	}

	let head_ref = try!(try!(repo.head()).resolve());
	let current_branch = match head_ref.shorthand() {
		Some(name) => name,
		None       => "",
	};

	Ok(VCSInfo {
		clean: status_list.entry_count() == 0,
		branch: current_branch.to_string(),
	})
}
