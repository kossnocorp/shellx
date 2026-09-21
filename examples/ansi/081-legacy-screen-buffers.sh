#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Older alternate-buffer modes: ?47 and ?1047' 'Their clearing/cursor semantics differ from ?1049 and vary by terminal.'
defer $'\e[?47l\e[?1047l\e[?1048l'
emit '\e[?1048h'
for mode in 47 1047; do
  printf '\033[?%sh\033[2J\033[H' "$mode"
  printf 'Alternate buffer selected using ?%sh\nReturning in two seconds.\n' "$mode"
  delay 2
  printf '\033[?%sl' "$mode"
done
emit '\e[?1048l'
note 'Returned. ?1048h/l separately saved/restored the cursor.'
