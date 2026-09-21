#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Palette entry: OSC 4;index;color ST; OSC 104;index ST' 'Changing palette entry 1 may recolor already printed red text.'
save_color '4;1' $'\e]104;1\e\\'
printf '\033[31mThis uses palette entry 1 (red).\033[0m\n'; delay
emit '\e]4;1;rgb:00/ff/88\e\\'; note 'Entry 1 is now mint green.'; delay 2
emit '\e]104;1\e\\'; note 'Entry 1 reset to configured default; original queried value restored on exit.'
