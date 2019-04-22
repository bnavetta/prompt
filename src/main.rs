use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod git;
mod preprompt;
mod prompt;
mod zsh;

#[derive(StructOpt)]
enum PromptArgs {
    #[structopt(name = "preprompt")]
    /// Prints the preprompt line
    Preprompt {
        #[structopt(short = "f")]
        /// Display full (expensive to compute) information
        full: bool,

        #[structopt(short = "c", default_value = "0")]
        /// Duration of the last command execution, in seconds
        command_duration: u64,
    },

    #[structopt(name = "prompt")]
    /// Prints the main line of the prompt
    Prompt {
        #[structopt(short = "e", default_value = "0")]
        /// The exit status of the last command run
        exit_status: i32,
    },

    #[structopt(name = "zsh-config")]
    /// Prints out ZSH configuration for the prompt
    ZshConfig,
}

const ZSH_CONFIG: &'static str = include_str!("../prompt_ben.zsh");

pub fn main() {
    let args = PromptArgs::from_args();

    match args {
        PromptArgs::Prompt { exit_status } => prompt::display_prompt(exit_status),
        PromptArgs::Preprompt {
            full,
            command_duration,
        } => preprompt::display_preprompt(full, command_duration),
        PromptArgs::ZshConfig => {
            let prompt_exe = env::current_exe().unwrap_or(PathBuf::from(stringify!(cfg_attr!(crate_name))));
            let config = ZSH_CONFIG.replace("%PROMPT_EXE%", &format!("{}", prompt_exe.display()));
            println!("{}", config);
        },
    };
}
