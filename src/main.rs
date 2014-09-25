#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate term;
extern crate time;

use docopt::FlagParser;
use std::time::duration::Duration;
use std::io::timer;
// use prompt::util;

mod prompt;

docopt!(Args, "
Usage:
    prompt preexec <command>
    prompt -h | --help
    prompt --version

Options:
    -h, --help        Show this message
    --version         Show version
    --exit=<status>   Exit status of the last command executed
", flag_exit: Option<int>)

fn main()
{
    let start = time::get_time().sec;

	let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

	let mut t = term::stdout().unwrap();

	t.fg(term::color::GREEN).unwrap();
	(write!(t, "{}", args)).unwrap();
	t.reset().unwrap();

    match prompt::util::is_ssh() {
        true  => println!("Running in SSH"),
        false => println!("Not running in SSH")
    }

    timer::sleep(Duration::seconds(5));

    let end = time::get_time().sec;

    println!("Time: {}", prompt::util::human_time(Duration::seconds(end - start)));
}
