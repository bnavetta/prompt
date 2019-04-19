use std::env;
use std::io::{self, Write};

use ansi_term::{ANSIString, ANSIStrings, Color};
use nix::unistd;
use whoami;

use super::zsh::ZSHEscaper;

const GRAY: Color = Color::Fixed(242);
const ORANGE: Color = Color::Fixed(166);

const PROMPT_SYMBOL: &'static str = "❯";
const ROOT_SYMBOL: &'static str = "#";

/// Determine if we're running over a SSH connection
fn in_ssh() -> bool {
    env::var("SSH_CONNECTION").is_ok()
}

/// TODO: check for MOSH as well

/// Determine whether or not the current user is *effectively* root
fn is_root() -> bool {
    unistd::geteuid().is_root()
}

pub fn display_prompt<'a>(exit_status: i32) {
    let host_color = if in_ssh() {
        Color::Purple
    } else {
        Color::Yellow
    };

    // See https://www.tldp.org/LDP/abs/html/exitcodes.html
    let prompt_color = match exit_status {
        0 => Color::Green,
        127 => Color::Yellow,
        130 => ORANGE,
        _ => Color::Red,
    };

    let parts = &[
            Color::White.paint("["),
            GRAY.paint(whoami::username()),
            Color::White.paint("@"),
            host_color.paint(whoami::host()),
            Color::White.paint("] "),
            if is_root() {
                prompt_color.paint(ROOT_SYMBOL)
            } else {
                ANSIString::from("")
            },
            prompt_color.paint(PROMPT_SYMBOL),
        ];

    let mut out = ZSHEscaper::new(io::stdout());
    let _ = write!(out, "{}", ANSIStrings(parts));
}
