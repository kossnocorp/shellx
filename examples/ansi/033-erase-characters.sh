#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Erase characters: CSI 5 X' 'Blanks five cells without shifting the remainder. | marks unchanged cursor.'
printf 'ABCDEFGHIJKLMNO'; delay
emit '\e[4G\e[5X'; printf '|\n'
