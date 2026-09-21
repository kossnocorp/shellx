#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Default foreground/background: OSC 10/11; reset 110/111' 'These can recolor the whole screen. Original queried colors return on exit.'
save_color 10 $'\e]110\e\\'
save_color 11 $'\e]111\e\\'
emit '\e]10;rgb:ff/dd/99\e\\\e]11;rgb:18/28/40\e\\'
note 'Warm foreground on a dark blue background.'; delay 2
emit '\e]110\e\\\e]111\e\\'
note 'Reset to configured defaults.'
