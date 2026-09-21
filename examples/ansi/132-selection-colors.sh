#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Selection colors: OSC 17/19; reset 117/119' 'Select this text with your mouse while the demo is waiting.'
save_color 17 $'\e]117\e\\'
save_color 19 $'\e]119\e\\'
# Demonstrate both resets on exit, before replaying the original queried colors.
defer $'\e]117\e\\\e]119\e\\'
emit '\e]17;rgb:99/33/aa\e\\\e]19;rgb:ff/ff/ff\e\\'
note 'A supported terminal should show a purple selection with white text.'
note 'On exit OSC 117/119 reset these colors, then the saved colors return if available.'
