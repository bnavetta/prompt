extern crate prompt;

use std::env;
use std::io;
use std::io::Write;

use prompt::style;
use prompt::style::Color;

const ORANGE: style::Color = Color::Other("166");

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

    let in_mux = prompt::mux::in_multiplexer();
    {
        let mut w = prompt::style().fg(if in_mux { Color::Blue } else { Color::Default }).go();
        try!(write!(w, "["));
    }

    {
        let mut w = prompt::style().fg(Color::Magenta).go();
        try!(write!(w, "{}", prompt::user::username().unwrap()));
    }

    print!("@");

    {
        let mut w = prompt::style().fg(if prompt::net::is_ssh() { ORANGE } else { Color::Yellow }).go();

        try!(match prompt::net::hostname() {
            Some(host) => write!(w, "{}", host),
            None       => write!(w, "localhost"),
        });
    }

    {
        let mut w = prompt::style().fg(if in_mux { Color::Blue } else { Color::Default }).go();
        try!(write!(w, "]"));
    }

    if prompt::user::is_root() {
        print!("⚡");
    }
    else {
        print!(" ");
    }

    {
        let mut w = prompt::style().fg(Color::Blue).go();
        try!(write!(w, "{} ", prompt::cwd()));
    }

    {
        let mut w = prompt::style().fg(if exit_status == 0 { Color::Green } else { Color::Red }).bold().go();
        try!(write!(w, "❯ "));
    }

    Ok(())
}

fn rprompt(job_count: u32) -> io::Result<()> {
    if job_count != 0 {
        let mut w = prompt::style().fg(Color::Cyan).go();
        try!(write!(w, "⚙ "));
    }

	  let cwd = env::current_dir().unwrap();
	  let vcs = prompt::vcs::repo(&cwd);

	  if vcs.is_ok() {
		    let repo = vcs.unwrap();

        let color = match repo.has_changes() {
            Ok(true) | Err(_) => Color::Red,
            Ok(false) => {
                match repo.is_synchronized() {
                    Ok(false) | Err(_) => Color::Yellow,
                    Ok(true) => Color::Green,
                }
            }
        };

        let mut w = prompt::style().fg(color).go();
        try!(write!(w, "{}", repo.symbol()));

        try!(match repo.current_branch() {
            Ok(branch) => write!(w, "{}", branch),
            Err(_)     => write!(w, "unknown"),
        });
	  }

	  Ok(())
}
