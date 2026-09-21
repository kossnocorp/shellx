#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Erase scrollback: CSI 3 J' 'This removes saved terminal scrollback where supported; it cannot be restored.'
delay 1
for ((i=1; i<=40; i++)); do printf 'Scrollback demo line %02d\n' "$i"; done
delay 1; emit '\e[3J'
note 'CSI 3 J sent. Try scrolling upward: saved history may be gone.'
