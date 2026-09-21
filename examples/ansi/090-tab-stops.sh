#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Custom tab stops: ESC H, CSI 0/3 g, CSI n I/Z' 'Sets stops at columns 5, 15, 25. Exit reinstalls conventional 8-column stops.'
cols=$(tput cols)
restore=$'\e7\e[3g'
for ((col=9; col<=cols; col+=8)); do restore+="${CSI}${col}G${ESC}H"; done
restore+=$'\e8'
defer "$restore"
emit '\e[3g'
for col in 5 15 25; do printf '\033[%sG\033H' "$col"; done
printf '\rA\tB\tC\tD\n'
printf '\r\033[2Iforward two stops'; printf '\r\033[25G\033[1ZBACK\n'
emit '\e[15G\e[0g'
printf '\rA\tB\tC (column 15 stop cleared)\n'
