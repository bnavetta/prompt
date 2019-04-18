A prompt in [Rust](http://www.rust-lang.org). Because.


## Redesign

* Prompt character is "❯", colored based on the exit status of the last command and prefixed with "#" if root
* Prompt line shows username and host in brackets, color-coded for SSH
* Prompt line has path and git status
    * Git branch name is red if dirty
    * Arrows for unpushed / unpulled changes (async)
    * Path is shortened with tico (if longer than some threshold?)
* Format:
```
/path/to/cwd branch* [arrows] [time of last command]
[user@host] ❯
```

To work reliably, it should use variables like `$prompt_newline`. That makes things a bit more complicated, because it's
not enough to just echo the whole prompt. Instead, have a few subcommands and some shell scripting to splice them together:

* A `preprompt` command which prints out the first line (path, VCS info, runtime of last command). This will also have
  an `--expensive` flag, which does extra checks (like for Git arrows). The shell wrapper will run this initially
  without `--expensive` to create the preprompt, then run it again in the background to fill it out.
* A `prompt` command which prints out the second line (user, host, SSH status, prompt character)

Use something like `prompt_pure_preprompt_render` to combine the two and reset the prompt as needed. Basically, the
initial pass runs in a ZSH `precmd` hook and sets `PROMPT` to the output of the `preprompt` and `prompt` commands,
joined with `$prompt_newline`. It also runs `preprompt --expensive` in the background. When that finishes, it replaces
the `preprompt` output with the `preprompt --expensive` output.