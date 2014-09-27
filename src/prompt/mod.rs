pub use self::hook::{prompt,rprompt,preexec};
pub use self::platform::{username,hostname};

pub mod util;
pub mod term;
mod hook;
mod platform;
