pub use self::hook::{prompt,rprompt,preexec};
pub use self::platform::{username,hostname};

pub mod util;
mod hook;
mod platform;
