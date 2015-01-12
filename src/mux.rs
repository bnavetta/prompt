//! Utilities for obtaining terminal multiplexer status
use std::fmt;
use std::os;

#[derive(Show, Copy)]
pub enum Multiplexer {
	Tmux,
	Screen
}

impl fmt::String for Multiplexer
{
	fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Multiplexer::Screen => fmt.write_str("screen"),
			Multiplexer::Tmux   => fmt.write_str("tmux"),
		}
	}
}

/// Determine the currently-running terminal multiplexer, if any.
pub fn multiplexer() -> Option<Multiplexer> {
	match os::getenv("TERM") {
		Some(term) => {
			if term.as_slice() == "screen" {
				match os::getenv("TMUX") {
					Some(_) => Some(Multiplexer::Tmux),
					None    => Some(Multiplexer::Screen)
				}
			}
			else {
				None
			}
		},
		None => None
	}
}

/// Determine whether or not a terminal multiplexer is currently running. If
/// knowing the specific multiplexer implementation is not required, this is
/// more efficient than `multiplexer`
pub fn in_multiplexer() -> bool {
	match os::getenv("TERM") {
		Some(term) => if term.as_slice() == "screen" { true } else { false },
		None       => false
	}
}
