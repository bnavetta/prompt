use std::os;

use super::git2::{Repository, Error, Reference};
use super::git::{Diff,diff_head_to_workdir};

pub struct VCSInfo
{
	pub branch: String,
	pub clean: bool
}

pub fn vcs_info() -> Result<VCSInfo, Error>
{
	let path = os::getcwd();

	let repo = try!(Repository::open(&path));
	let diff = try!(diff_head_to_workdir(&repo));

	let head_ref = try!(try!(repo.head()).resolve());
	let current_branch = match head_ref.shorthand() {
		Some(name) => name,
		None       => "",
	};

	Ok(VCSInfo {
		clean: diff.num_deltas() > 0,
		branch: current_branch.to_string(),
	})
}
