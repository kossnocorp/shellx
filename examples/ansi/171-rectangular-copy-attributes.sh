#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'DEC rectangular copy and attributes' 'DECCRA $v copies; DECCARA $r changes SGR; DECRARA $t toggles attributes.'
rows; delay
# Source top/left/bottom/right/page; destination top/left/page. Page 1.
emit '\e[5;1;7;15;1;16;25;1$v'; delay
emit '\e[8;5;10;25;1;4$r'; delay
emit '\e[11;5;13;25;7$t'
