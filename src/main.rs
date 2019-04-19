use std::env;
use std::path::Path;

use ansi_term::Color::{Blue, Cyan, Purple, White, Yellow};
use ansi_term::{ANSIStrings, Color};
use git2::Repository;
use tico::tico;
use whoami;

mod git;

const MAX_UNSHORTENED_PATH_LEN: usize = 20;
const GRAY: Color = Color::Fixed(242);

fn in_ssh() -> bool {
    env::var("SSH_CONNECTION").is_ok()
}

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

pub fn main() {
    let host_color = if in_ssh() { Purple } else { Yellow };
    let login_strings = &[
        White.paint("["),
        GRAY.paint(whoami::username()),
        White.paint("@"),
        host_color.paint(whoami::host()),
        White.paint("]"),
    ];

    let path_string = Blue.paint(path());

    println!("{} {}", ANSIStrings(login_strings), path_string);

    let repo = Repository::discover(".").unwrap();
    let head = self::git::get_head(&repo).unwrap();
    println!("HEAD = {}", head);

    let (ahead, behind) = git::fetch_current(&repo).unwrap();
    let mut arrows = vec![];
    if ahead > 0 {
        arrows.push(Cyan.paint("⇡"));
    }
    if behind > 0 {
        arrows.push(Cyan.paint("⇣"));
    }
    println!("{}", ANSIStrings(&arrows));

    println!("{} commits ahead, {} commits behind", ahead, behind);
}
