# Various bits from https://github.com/sindresorhus/pure

PROMPT_CMD="%PROMPT_EXE%"

prompt_ben_precmd() {
    local last_status=$?

    # check time before unsetting
    typeset -i command_duration
    (( command_duration = EPOCHSECONDS - ${prompt_ben_command_timestamp:-$EPOCHSECONDS} ))
    unset prompt_pure_cmd_timestamp

    # async updates

    prompt_ben_preprompt="$($PROMPT_CMD preprompt -c $command_duration)"
    prompt_ben_prompt="$($PROMPT_CMD prompt -e $last_status)"
}

prompt_ben_preexec() {
    #TODO: cancel async update if doing a fetch/pull

    typeset -ig prompt_ben_command_timestamp=$EPOCHSECONDS
}

prompt_ben_setup() {
    # Prevent percentage showing up if output doesn't end with a newline.
    export PROMPT_EOL_MARK=''

    if [[ -z $prompt_newline ]]; then
        # This variable needs to be set, usually set by promptinit.
        typeset -g prompt_newline=$'\n%{\r%}'
    fi

    zmodload zsh/datetime

    autoload -Uz add-zsh-hook

    add-zsh-hook precmd prompt_ben_precmd
    add-zsh-hook preexec prompt_ben_preexec

    typeset -g prompt_ben_preprompt="$($PROMPT_CMD preprompt)"
    typeset -g prompt_ben_prompt="$($PROMPT_CMD prompt)"

    PROMPT='${prompt_ben_preprompt}${prompt_newline}${prompt_ben_prompt} '

    unset ZSH_THEME
}

prompt_ben_setup "$@"