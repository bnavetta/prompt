extern crate prompt;

fn main() {
	match prompt::mux::multiplexer() {
		Some(m) => println!("Multiplexer: {}", m),
		None    => println!("No multiplexer"),
	}

	println!("{} {}!",
		if prompt::user::is_root() { "Greetings, master" } else { "Hello," },
		prompt::user::username().unwrap());

	println!("Happily running on {} in {}", prompt::net::hostname(), prompt::cwd().display());
}
