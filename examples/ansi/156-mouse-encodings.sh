#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Mouse encodings: legacy / UTF-8 / SGR / urxvt / SGR pixels' 'Click at different positions during each five-second segment.'
for mode in 0 1005 1006 1015 1016; do
  note "Encoding mode $mode (0 = legacy ESC [ M + encoded bytes)"
  if ((mode)); then printf '\033[?%sh' "$mode"; fi
  emit '\e[?1000h'; events 5; emit '\e[?1000l'
  if ((mode)); then printf '\033[?%sl' "$mode"; fi
done
