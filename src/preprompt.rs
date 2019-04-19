use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

use ansi_term::{ANSIString, ANSIStrings, Color};
use dirs;
use failure::Error;
use git2::Repository;
use humantime::format_duration;
use tico::tico;

use super::git;
use super::zsh::ZSHEscaper;

const COMMAND_TIME_THRESHOLD: Duration = Duration::from_secs(10);
const MAX_UNSHORTENED_PATH_LEN: usize = 20;

const UP_ARROW_SYMBOL: &'static str = "⇡";
const DOWN_ARROW_SYMBOL: &'static str = "⇣";

fn path() -> String {
    if let Ok(current_dir) = env::current_dir() {
        let resolved = match dirs::home_dir() {
            Some(home_dir) => match current_dir.strip_prefix(home_dir) {
                Ok(truncated) => Path::new("~").join(truncated),
                Err(_) => current_dir,
            },
            None => current_dir,
        };

        let path = resolved.to_string_lossy();
        if path.len() > MAX_UNSHORTENED_PATH_LEN {
            tico(&path)
        } else {
            path.to_string()
        }
    } else {
        String::new()
    }
}

fn add_git_info<'a>(full: bool, parts: &mut Vec<ANSIString<'a>>) -> Result<(), Error> {
    let repo = Repository::discover(".")?;

    let head = git::get_head(&repo)?;
    let is_dirty = git::is_dirty(&repo)?;

    parts.push(ANSIString::from(" "));

    // Worth distinguishing between staged and unstaged?
    let head_color = if is_dirty { Color::Red } else { Color::Green };
    parts.push(head_color.paint(format!("±{}", head)));

    if full {
        let (ahead, behind) = git::fetch_current(&repo)?;

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
    let mut parts = vec![Color::Blue.paint(path())];

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
