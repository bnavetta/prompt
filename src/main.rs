#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

extern crate term;

use docopt::FlagParser;
// use prompt::util;

mod prompt;

docopt!(Args, "
Usage: prompt [options]

Options:
    -h, --help         Show this message
    --exit STATUS     Pass the last exit status into the program
", flag_exit: Option<int>)

fn main()
{
	let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

	let mut t = term::stdout().unwrap();

	t.fg(term::color::GREEN).unwrap();
	t.bg(term::color::BLACK).unwrap();
	(write!(t, "{}", args)).unwrap();
	t.reset().unwrap();
}
