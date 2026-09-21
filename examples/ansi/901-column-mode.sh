#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Legacy 132/80 columns: CSI ?3 h/l' 'May resize and clear the screen. Original character dimensions are restored on exit.'
read -r height width < <(stty size)
defer "${CSI}8;${height};${width}t"
defer $'\e[?3l'
delay 1; emit '\e[?3h'; note '132-column mode requested (?3h)'; delay 2
emit '\e[?3l'; note '80-column mode requested (?3l)'
