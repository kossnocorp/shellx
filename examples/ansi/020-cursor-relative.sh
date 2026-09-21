#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Relative cursor movement' 'A/B/C/D = up/down/right/left; E/F = next/previous line at column 1.'
at 6 20; printf 'start'; delay
emit '\e[2A'; printf 'A up'; delay
emit '\e[4B'; printf 'B down'; delay
emit '\e[8C'; printf 'C right'; delay
emit '\e[20D'; printf 'D left'; delay
emit '\e[3E'; printf 'E next-line'; delay
emit '\e[1F'; printf 'F previous-line'
at 15 1; printf 'HPR: '; emit '\e[5a'; printf 'a = right 5'
at 16 1; printf 'VPR:'; emit '\e[2e'; printf 'e = down 2'
