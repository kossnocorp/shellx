#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Insert/delete lines: CSI 2 L / CSI 2 M' 'Restricted to rows 5–14. Watch the numbered rows shift.'
rows; emit '\e[5;14r'; at 8 1; delay
emit '\e[2L'; printf 'two lines inserted'; delay 1.5
at 11 1; emit '\e[2M'; delay
