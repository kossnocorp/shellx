#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Taskbar/tab progress: OSC 9;4;state;percent ST' 'Watch the terminal icon/tab: normal, error, indeterminate, paused, then cleared.'
defer $'\e]9;4;0;0\e\\'
for percent in 0 25 50 75 100; do
  printf '\033]9;4;1;%s\033\\\rProgress %s%%' "$percent" "$percent"; delay
done
printf '\n'
for entry in '2 error' '3 indeterminate' '4 paused'; do
  note "State $entry"; printf '\033]9;4;%s;50\033\\' "${entry%% *}"; delay 1
done
emit '\e]9;4;0;0\e\\'; note 'Progress cleared.'
