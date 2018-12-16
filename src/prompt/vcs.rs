use std::os;

use super::git2::{Repository, Error};
use super::git::{StatusList, StatusOptions, OPT_INCLUDE_IGNORED};

pub struct VCSInfo
{
	pub branch: String,
	pub clean: bool
}

pub fn vcs_info() -> Result<VCSInfo, Error>
{
	let path = os::getcwd();

	let mut options = StatusOptions::defaults();
	options.flags.remove(OPT_INCLUDE_IGNORED);

	let repo = Repository::open(&path)?;
	let status_list = StatusList::new(&repo, options)?;

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
