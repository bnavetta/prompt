pub use self::hook::{prompt,rprompt,preexec,precmd};
pub use self::platform::{username,hostname};

pub mod config;
pub mod util;
pub mod term;
mod hook;
mod platform;
