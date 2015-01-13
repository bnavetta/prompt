extern crate prompt;
extern crate term;

use std::os;

fn main() {
	match prompt::mux::multiplexer() {
		Some(m) => println!("Multiplexer: {}", m),
		None    => println!("No multiplexer"),
	}

	println!("{} {}!",
		if prompt::user::is_root() { "Greetings, master" } else { "Hello," },
		prompt::user::username().unwrap());

	println!("Happily running on {} in {}", prompt::net::hostname(), prompt::cwd().display());

	let mut t = term::stdout().unwrap();

	rprompt(&mut *t);
}

fn precmd() {

}

fn main_prompt() {

}

fn rprompt(t: &mut term::Terminal<term::WriterWrapper>) {
	let cwd = os::getcwd().unwrap();
	match prompt::vcs::repo(&cwd) {
		Some(repo) => {
			let color = if repo.has_changes() {
				term::color::RED
			}
			else if !repo.is_synchronized() {
				term::color::YELLOW
			}
			else {
				term::color::GREEN
			};

			t.fg(color).unwrap();
			(writeln!(t, "{}{}", repo.current_branch(), repo.symbol())).unwrap();
			t.reset().unwrap();
		},
		None => {},
	}
}
