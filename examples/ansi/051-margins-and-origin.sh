#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Scrolling margins and origin mode' 'CSI 5;14 r sets margins; ?6h makes CUP relative to the region.'
rows; emit '\e[5;14r\e[?6h\e[1;1H'; printf 'Origin-relative row 1 = screen row 5'; delay
emit '\e[?6l\e[16;1H'; printf 'Origin off: row 16 = screen row 16'
emit '\e[r'
