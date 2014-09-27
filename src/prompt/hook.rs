use std::os;
use super::term;
use super::util;

pub fn preexec(command: &str)
{
	//print!("\x1B]0;{}: {}\x07", os::getcwd().display(), command);
}

pub fn preprompt(execution_seconds: int) {

}

pub fn prompt(exit_status: int) {
	print!("{}", term::foreground("❯", if exit_status == 0 { term::Green } else { term::Magenta }));
	print!(" ");
}

pub fn rprompt() {
	print!("{}", term::foreground("hi", term::Green));
}
