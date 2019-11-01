use std::io::{self, Write};
use std::time::Duration;

use ansi_term::{ANSIString, ANSIStrings, Color};
use failure::Error;
use git2::Repository;
use humantime::format_duration;

use super::git;
use super::paths::current_directory;
use super::zsh::ZSHEscaper;

const COMMAND_TIME_THRESHOLD: Duration = Duration::from_secs(10);

const UP_ARROW_SYMBOL: &str = "⇡";
const DOWN_ARROW_SYMBOL: &str = "⇣";

fn add_git_info<'a>(full: bool, parts: &mut Vec<ANSIString<'a>>) -> Result<(), Error> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        // Don't consider directories not being git repos an error, since it clutters up
        // debug-mode output
        Err(ref e) if e.code() == git2::ErrorCode::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    let head = git::get_head(&repo)?;
    let is_dirty = git::is_dirty(&repo)?;

    parts.push(ANSIString::from(" "));

    // Worth distinguishing between staged and unstaged?
    let head_color = if is_dirty { Color::Red } else { Color::Green };
    parts.push(head_color.paint(format!("±{}", head)));

    if full {
        let (ahead, behind) = git::fetch_current(&repo)?;

        if ahead + behind > 0 {
            parts.push(ANSIString::from(" "))
        }

        if ahead > 0 {
            parts.push(Color::Cyan.paint(UP_ARROW_SYMBOL));
        }

        if behind > 0 {
            parts.push(Color::Cyan.paint(DOWN_ARROW_SYMBOL));
        }
    }

    Ok(())
}

pub fn display_preprompt(full: bool, command_duration_secs: u64) {
    let mut parts = vec![Color::Blue.paint(current_directory())];

    if let Err(e) = add_git_info(full, &mut parts) {
        if cfg!(debug_assertions) {
            eprintln!("Git error: {}", e);
        }
    }

    let command_duration = Duration::from_secs(command_duration_secs);
    if command_duration > COMMAND_TIME_THRESHOLD {
        parts.push(ANSIString::from(" "));
        parts.push(Color::Yellow.paint(format_duration(command_duration).to_string()));
    }

    let mut out = ZSHEscaper::new(io::stdout());
    let _ = write!(out, "{}", ANSIStrings(&parts));
}
