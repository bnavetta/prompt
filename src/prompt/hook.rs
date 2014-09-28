use std::os;
use std::time::duration::Duration;
use super::config;
use super::term;
use super::util;

pub fn preexec(command: &str)
{
	//print!("\x1B]0;{}: {}\x07", os::getcwd().display(), command);
}

pub fn precmd(execution_seconds: i64) {
	let execution_time = Duration::seconds(execution_seconds);

	if execution_time > config::max_exec_time() {
		let time = util::human_time(execution_time);
		println!("{}", term::foreground(time.as_slice(), config::COLOR_EXEC_TIME));
		// print!("\n{}s\n", execution_time.num_seconds());
	}
}

pub fn prompt(exit_status: int) {
	print!("{}", term::foreground("❯",
		if exit_status == 0 { config::COLOR_SUCCESS } else { config::COLOR_FAILURE }));
	print!(" ");
}

pub fn rprompt() {
	print!("{}", term::foreground("hi", term::Green));
}
