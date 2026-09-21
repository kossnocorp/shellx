#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Left/right margins: ?69h + CSI 10;45 s' 'Extension: many terminals ignore this. Text should wrap inside columns 10–45.'
rows
emit '\e[5;14r\e[?69h\e[10;45s\e[?6h\e[1;1H'
printf '%s' 'This long sentence should wrap within a rectangular region, preserving content outside its horizontal and vertical margins. '
emit '\e[?6l\e[?69l\e[r'
