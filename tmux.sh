#!/bin/sh

SESSION=prompt

tmux new-session -d -s $SESSION

tmux new-window -t $SESSION:1 -n 'Main'
tmux split-window -h

tmux select-pane -t 0
tmux send-keys "vim" C-m

tmux select-pane -t 1
tmux split-window -v

tmux select-pane -t 1
tmux send-keys "zsh" C-m
tmux send-keys "fpath=( $PWD $fpath)" C-m
tmux send-keys "path=( $PWD/target $path)" C-m
tmux send-keys "autoload -U promptinit && promptinit && prompt ben" C-m

tmux select-window -t $SESSION:1
tmux select-pane -t 2 # start in the terminal
tmux attach-session -t $SESSION

