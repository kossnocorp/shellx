#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'DEC line dimensions: ESC # 3/4/5/6' 'Double-height needs the same text printed on the upper and lower line.'
at 5 1; emit '\e#6'; printf 'DOUBLE WIDTH'
at 8 1; emit '\e#3'; printf 'DOUBLE HEIGHT'
at 9 1; emit '\e#4'; printf 'DOUBLE HEIGHT'
at 12 1; emit '\e#5'; printf 'Normal single-width / single-height line'
