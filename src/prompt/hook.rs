extern crate term;

use std::os;
use term::color;

pub fn preexec(command: &str)
{
	//print!("\x1B]0;{}: {}\x07", os::getcwd().display(), command);
}

pub fn preprompt() {

}

pub fn prompt(exit_status: int) {
	let mut t = term::stdout().unwrap();
	if exit_status == 0
	{
		t.fg(color::BRIGHT_MAGENTA).unwrap();
		// print!("%F{{magenta}}");
	}
	else
	{
		t.fg(color::BRIGHT_RED).unwrap();
		// print!("%F{{red}}");
	}
	write!(t, "❯");
	// t.reset().unwrap();
	write!(t, "%f ");
	// print!("❯%f ");
}

pub fn rprompt() {
	let mut t = term::stdout().unwrap();
	// t.fg(color::GREEN).unwrap();
	write!(t, "%F{{green}}Hi");
	// t.reset().unwrap();
	write!(t, "%f");
}
