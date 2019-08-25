# Various bits from https://github.com/sindresorhus/pure

PROMPT_CMD="%PROMPT_EXE%"

prompt_ben_precmd() {
    local last_status=$?

    # check time before unsetting
    typeset -gi prompt_ben_command_duration
    (( prompt_ben_command_duration = EPOCHSECONDS - ${prompt_ben_command_timestamp:-$EPOCHSECONDS} ))
    unset prompt_pure_cmd_timestamp

    # async updates
    prompt_ben_async_tasks

    prompt_ben_preprompt="$($PROMPT_CMD preprompt -c $prompt_ben_command_duration)"
    prompt_ben_prompt="$($PROMPT_CMD prompt -e $last_status)"
}

prompt_ben_async_tasks() {
    # Start the async worker
    if ((!${prompt_ben_async_init:-0})); then
        async_start_worker "prompt_ben" -u
        async_register_callback "prompt_ben" prompt_ben_async_callback
        typeset -g prompt_ben_async_init=1
    fi

    # Update the async worker's current working directory
    async_worker_eval "prompt_ben" builtin cd -q $PWD

    # TODO: time limit on how often dirty checking runs?
    async_job "prompt_ben" prompt_ben_async_update_preprompt
}

prompt_ben_async_callback() {
    local job=$1 code=$2 output=$3 exec_time=$4 next_pending=$6
    local rerender=0

    case $job in
        \[async])
            if [[ $code -eq 2 ]]; then
                # the worker died unexpectedly
                typeset -g prompt_ben_async_init=0
            fi
            ;;
        prompt_ben_async_update_preprompt)
            if [[ -n "$output" ]]; then
                typeset -g prompt_ben_preprompt="$output"
                rerender=1
            fi
            ;;
    esac

    # If there are other pending async jobs, delay rerendering until they're all processed
    # Use the prompt_ben_async_render_requested variable to keep track across callback invocations
    if (( next_pending )); then
        if (( rerender )); then
            typeset -g prompt_ben_async_render_requested=1
        fi
        return
    fi

    if [[ ${prompt_ben_async_render_requested:-$rerender} = 1 ]]; then
        zle && zle .reset-prompt
    fi
    unset prompt_ben_async_render_requested
}

prompt_ben_async_update_preprompt() {
    $PROMPT_CMD preprompt -c $prompt_ben_command_duration -f
}

prompt_ben_preexec() {
    #TODO: cancel async update if doing a fetch/pull

    typeset -ig prompt_ben_command_timestamp=$EPOCHSECONDS
}

# enable substitution in prompt environment variables
setopt promptsubst

prompt_ben_setup() {
    # Prevent percentage showing up if output doesn't end with a newline.
    export PROMPT_EOL_MARK=''

    if [[ -z $prompt_newline ]]; then
        # This variable needs to be set, usually set by promptinit.
        typeset -g prompt_newline=$'\n%{\r%}'
    fi

    zmodload zsh/datetime
    zmodload zsh/zle

    autoload -Uz add-zsh-hook

    add-zsh-hook precmd prompt_ben_precmd
    add-zsh-hook preexec prompt_ben_preexec

    typeset -g prompt_ben_preprompt="$($PROMPT_CMD preprompt)"
    typeset -g prompt_ben_prompt="$($PROMPT_CMD prompt)"

    # Avoids an undefined variable error on systems not using iTerm2
    if [ "x$TERM_PROGRAM" = "iTerm.app" ]; then
        PROMPT='${prompt_ben_preprompt}${prompt_newline}%{$(iterm2_prompt_mark)%}${prompt_ben_prompt} '
    else
        PROMPT='${prompt_ben_preprompt}${prompt_newline}${prompt_ben_prompt} '
    fi

    unset ZSH_THEME
}

prompt_ben_setup "$@"