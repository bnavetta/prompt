#![feature(unsafe_destructor)]
#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use docopt::FlagParser;
// use prompt::util;

mod prompt;

docopt!(Args, "
Usage:
    prompt_ben prompt --exit=<status>
    prompt_ben rprompt [options]
    prompt_ben preexec <command>
    prompt_ben precmd --exec-time=<time>
    prompt_ben -h | --help
    prompt_ben --version

Options:
    -h, --help          Show this message
    --version           Show version
    --exit=<status>     Exit status of the last command executed
	--exec-time=<time>  Time taken to execute the last command
", flag_exit: Option<int>, flag_exec_time: Option<int>)

fn main()
{
	let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    prompt::git::test();

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
	else if args.cmd_precmd {
		prompt::precmd(args.flag_exec_time.unwrap() as i64);
	}
}
