#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'xterm modifyOtherKeys: CSI >4;2 m' 'Press modified keys (Ctrl/Alt/Shift combinations). Returned encodings vary.'
defer $'\e[>4;0m'
emit '\e[>4;2m'; events
emit '\e[>4;0m'
