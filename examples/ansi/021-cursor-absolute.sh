#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Absolute cursor positioning' 'CUP H / HVP f use row;column. CHA G / HPA ` set column. VPA d sets row.'
emit '\e[5;10H'; printf 'H: row 5 col 10'
emit '\e[7;10f'; printf 'f: row 7 col 10'
emit '\e[25G'; printf 'G: col 25'
emit '\e[10d'; printf 'd: row 10, same column'
emit '\e[4`'; printf '` : col 4'
delay
emit '\e[H'; printf 'HOME (CSI H)'
