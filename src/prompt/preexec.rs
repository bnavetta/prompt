use std::os;

pub fn preexec(command: &str) -> String
{
	return format!("\x1B]0;{}: {}\x07", os::getcwd().display(), command);
}
