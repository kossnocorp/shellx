#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Cursor visibility, blinking, and shape' 'Watch the cursor at the right of each label; each shape stays for one second.'
defer $'\e[0 q\e[?12h\e[?25h'
for ((shape=0; shape<=6; shape++)); do
  printf '\r\033[2KCSI %s SPACE q: cursor here -> \033[%s q' "$shape" "$shape"; delay 1
done
printf '\r\033[2K?25l: hidden cursor -> '; emit '\e[?25l'; delay 1
emit '\e[?25h'; printf '\r\033[2K?25h: visible cursor -> '; delay 1
emit '\e[?12l'; printf '\r\033[2K?12l: blink disabled -> '; delay 1
emit '\e[?12h'; printf '\r\033[2K?12h: blink enabled -> '; delay 1
printf '\n'
