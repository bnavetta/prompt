extern crate prompt;
extern crate term;

use std::env;
use std::io;

fn main() {
	match prompt::mux::multiplexer() {
		Some(m) => println!("Multiplexer: {}", m),
		None    => println!("No multiplexer"),
	}

	println!("{} {}!",
		if prompt::user::is_root() { "Greetings, master" } else { "Hello," },
		prompt::user::username().unwrap());

	println!("Happily running on {} in {}", prompt::net::hostname().unwrap(), prompt::cwd());

	let mut t = term::stdout().unwrap();

	rprompt(&mut *t).unwrap();
}

fn precmd() {

}

fn main_prompt() {

}

fn rprompt<W: io::Write>(t: &mut term::Terminal<W>) -> io::Result<()> {
	let cwd = env::current_dir().unwrap();
	let vcs = prompt::vcs::repo(&cwd);

	if vcs.is_ok() {
		let repo = vcs.unwrap();
		let color = if repo.has_changes().unwrap() {
			term::color::RED
		}
		else if !repo.is_synchronized().unwrap() {
			term::color::YELLOW
		}
		else {
			term::color::GREEN
		};

		try!(t.fg(color));
		try!(writeln!(t, "{}{}", repo.symbol(), repo.current_branch().unwrap()));
		try!(t.reset());
	}

	Ok(())
}
