#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate term;
extern crate time;

use docopt::FlagParser;
// use prompt::util;

mod prompt;

docopt!(Args, "
Usage:
    prompt_ben prompt --exit=<status>
    prompt_ben rprompt [options]
    prompt_ben preexec <command>
    prompt_ben -h | --help
    prompt_ben --version

Options:
    -h, --help        Show this message
    --version         Show version
    --exit=<status>   Exit status of the last command executed
", flag_exit: Option<int>)

fn main()
{
	let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("prompt {}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
    }
	else if args.cmd_prompt {
		prompt::prompt(args.flag_exit.unwrap());
	}
	else if args.cmd_rprompt {
		prompt::rprompt();
	}
    else if args.cmd_preexec {
        prompt::preexec(args.arg_command.as_slice());
    }

	// let mut t = term::stdout().unwrap();

	// t.fg(term::color::GREEN).unwrap();
	// (write!(t, "{}", args)).unwrap();
	// t.reset().unwrap();

    // match prompt::util::is_ssh() {
        // true  => println!("Running in SSH"),
        // false => println!("Not running in SSH")
    // }
}
