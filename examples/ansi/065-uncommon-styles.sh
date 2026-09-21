#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Less common SGR attributes' 'Unsupported attributes usually look like ordinary text.'
for pair in '20:23:Fraktur' '26:50:Proportional-spacing' '51:54:Framed' '52:54:Encircled' '53:55:Overline' '73:75:Superscript' '74:75:Subscript'; do
  on=${pair%%:*}; rest=${pair#*:}; off=${rest%%:*}; name=${rest#*:}
  printf '%-24s \033[%smSample\033[%sm reset\033[0m\n' "$name ($on/$off)" "$on" "$off"
done
for ((i=10; i<=19; i++)); do sample "$i" "Font selector $i: ABC abc 123"; emit '\e[10m'; done
for ((i=60; i<=64; i++)); do sample "$i" "Ideogram decoration $i"; emit '\e[65m'; done
defer $'\e[10;23;50;54;55;65;75;0m'
