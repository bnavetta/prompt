extern crate prompt;

use std::env;
use std::io;

const ORANGE: &'static str = "166";

fn main() {
    let mut args = env::args();

    args.next(); // First arg is program
    let mode: &str = &args.next().expect("Must specify prompt mode");
    match mode {
        "prompt"  => main_prompt(args.next().expect("Missing exit status").parse::<i32>().unwrap()).unwrap(),
        "rprompt" => rprompt(args.next().expect("Missing job count").parse::<u32>().unwrap()).unwrap(),
        _         => panic!("Illegal usage: {}", mode),
    }
}

fn main_prompt(exit_status: i32) -> io::Result<()> {
    let host_color = if prompt::net::is_ssh() { ORANGE } else { "yellow" };

    let in_mux = prompt::mux::in_multiplexer();
    if in_mux {
        print!("%F{{blue}}[%f");
    }
    else {
        print!("[");
    }

    print!("%F{{magenta}}{}%f", prompt::user::username().unwrap());

    print!("@");

    print!("%F{{{}}}{}%f", host_color, prompt::net::hostname().unwrap());

    if in_mux {
        print!("%F{{blue}}]%f");
    }
    else {
        print!("]");
    }

    if prompt::user::is_root() {
        print!("⚡");
    }
    else {
        print!(" ");
    }

    print!("%F{{blue}}{}%f ", prompt::cwd());

    let caret_color = if exit_status == 0 { "green" } else { "red" };
    print!("%F{{{}}}❯ %f", caret_color);

    Ok(())
}

fn rprompt(job_count: u32) -> io::Result<()> {
    if job_count != 0 {
        print!("%F{{cyan}}⚙%f ");
    }

	  let cwd = env::current_dir().unwrap();
	  let vcs = prompt::vcs::repo(&cwd);

	  if vcs.is_ok() {
		    let repo = vcs.unwrap();
		    let color = if repo.has_changes().unwrap() {
			      "red"
		    }
		    else if !repo.is_synchronized().unwrap() {
			      "yellow"
		    }
		    else {
			      "green"
		    };

        print!("%F{{{}}}{}{}%f", color, repo.symbol(), repo.current_branch().unwrap());
	  }

	  Ok(())
}
