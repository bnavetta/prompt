use std::env;
use std::path::Path;

use dirs;
use tico::tico;

const MAX_UNSHORTENED_PATH_LEN: usize = 20;

pub fn current_directory() -> String {
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
