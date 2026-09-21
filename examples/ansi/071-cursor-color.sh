#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Cursor color: OSC 12; color ST; reset OSC 112 ST' 'The original color is queried and restored when possible.'
save_color 12 $'\e]112\e\\'
for color in red green blue; do
  printf '\r\033[2KCursor color: %s -> ' "$color"
  printf '\033]12;%s\033\\' "$color"; delay 1
done
emit '\e]112\e\\'; note '  Reset to terminal default.'
