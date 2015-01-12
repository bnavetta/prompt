extern crate prompt;

use prompt::mux;
use prompt::net;
use prompt::user;

fn main() {
	match mux::multiplexer() {
		Some(m) => println!("Multiplexer: {}", m),
		None    => println!("No multiplexer"),
	}

	println!("{} {}!",
		if user::is_root() { "Greetings, master" } else { "Hello," },
		user::username().unwrap());

	println!("Happily running on {}", net::hostname());
}
