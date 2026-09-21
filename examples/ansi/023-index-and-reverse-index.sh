#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'IND / NEL / RI can scroll at margins' 'ESC D: down; ESC E: next line at column 1; ESC M: up.'
rows
emit '\e[5;14r'
at 14 5; delay; emit '\eD'; printf 'IND scrolled up'; delay
emit '\eE'; printf 'NEL scrolled up and returned to column 1'; delay
at 5 1; emit '\eM'; printf 'RI scrolled down'
