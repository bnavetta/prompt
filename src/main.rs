use std::env;
use std::path::Path;

use ansi_term::Color::{Blue, Cyan, Green, Purple, White, Yellow};
use ansi_term::{ANSIString, ANSIStrings, Color};
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
    // Start with login info
    let host_color = if in_ssh() { Purple } else { Yellow };
    let mut parts = vec![
        White.paint("["),
        GRAY.paint(whoami::username()),
        White.paint("@"),
        host_color.paint(whoami::host()),
        White.paint("] "),
    ];

    // Path
    parts.push(Blue.paint(path()));
    parts.push(ANSIString::from(" "));

    // Git info
    let repo = Repository::discover(".").unwrap();
    let head = self::git::get_head(&repo).unwrap();
    // TODO: dirty status
    let head_str = match head {
        self::git::Head::Branch(branch) => Green.paint(branch),
        self::git::Head::Commit(commit) => Purple.paint(format!("{:.8}", commit)),
    };
    parts.push(head_str);

    let (ahead, behind) = git::fetch_current(&repo).unwrap();
    if ahead > 0 {
        parts.push(Cyan.paint("⇡"));
    }
    if behind > 0 {
        parts.push(Cyan.paint("⇣"));
    }
    println!("{}", ANSIStrings(&parts));
}
