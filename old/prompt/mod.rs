extern crate libc;

#[phase(plugin)] extern crate git2;
extern crate libgit2;
extern crate git2;

pub use self::hook::{prompt,rprompt,preexec,precmd};

pub mod config;
pub mod util;
pub mod term;
pub mod git;
pub mod vcs;
mod hook;
