pub use self::hook::{prompt,rprompt,preexec,precmd};
pub use self::platform::{username,hostname};

pub mod config;
pub mod util;
pub mod term;
pub mod git;
mod hook;
mod platform;
