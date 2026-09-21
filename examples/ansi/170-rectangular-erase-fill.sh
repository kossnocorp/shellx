#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'DEC rectangular fill, erase, selective erase' 'DECFRA $x / DECERA $z / DECSERA ${. Coordinates: top;left;bottom;right.'
rows; delay
emit '\e[35;6;10;8;25$x'; delay 1
emit '\e[10;10;12;25$z'; delay 1
at 14 10; emit '\e[1"q'; printf 'PROTECTED'; emit '\e[0"q'; printf ' ordinary'
defer $'\e[0"q'
emit '\e[14;1;14;45${'
