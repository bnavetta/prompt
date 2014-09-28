use std::os;
use std::string::String;
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

	let mut preprompt = String::new();

	if util::is_ssh() {
		preprompt.push_str("[");
		preprompt.push_str(term::foreground("%n", config::COLOR_USER).as_slice());
		preprompt.push_str("@");
		preprompt.push_str(term::foreground("%m", config::COLOR_HOST).as_slice());
		preprompt.push_str("] ");
	}

	if execution_time > config::max_exec_time() {
		let time = util::human_time(execution_time);
		preprompt.push_str(term::foreground(time.as_slice(), config::COLOR_EXEC_TIME).as_slice());
	}

	println!("{}", preprompt);
}

pub fn prompt(exit_status: int) {
	print!("{} ", term::foreground("❯",
		if exit_status == 0 { config::COLOR_SUCCESS } else { config::COLOR_FAILURE }));
}

pub fn rprompt() {
	print!("{}", term::foreground("hi", term::Green));
}
