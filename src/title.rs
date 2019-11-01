use std::io::{self, Write};

use whoami;

use super::paths::current_directory;

const TITLE_START: &str = "\x1b]0;";
const TITLE_END: &str = "\x07";

pub fn update_title() {
    let mut out = io::stdout();
    let _ = write!(
        out,
        "{}{}@{}:{}{}",
        TITLE_START,
        whoami::username(),
        whoami::hostname(),
        current_directory(),
        TITLE_END
    );
    let _ = out.flush();
}
