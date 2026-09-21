#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Automatic wrapping: CSI ? 7 h/l' 'With wrapping off, additional characters overwrite the rightmost cell.'
cols=$(tput cols)
at 5 "$((cols-8))"; emit '\e[?7h'; printf 'WRAP: abcdefghijklmnop'
at 9 "$((cols-8))"; emit '\e[?7l'; printf 'NO WRAP: abcdefghijklmnop'
emit '\e[?7h'
