//! Utilities for obtaining terminal multiplexer status
use std::fmt;
use std::env;

use self::Multiplexer::*;

pub enum Multiplexer {
	Tmux,
	Screen
}

impl fmt::Display for Multiplexer
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
	env::var("TERM").ok().and_then(|term| {
		if &term[..] == "screen" {
			Some(if env::var("TMUX").is_ok() { Tmux } else { Screen })
		}
		else {
			None
		}
	})
}

/// Determine whether or not a terminal multiplexer is currently running.
pub fn in_multiplexer() -> bool {
	match env::var("TERM") {
		Ok(term) => if &term[..] == "screen" { true } else { false },
		Err(_)   => false
	}
}
