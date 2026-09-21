#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Terminal newline mode: CSI 20 h/l' 'OS ONLCR translation is disabled here to expose terminal behavior.'
stty -onlcr
emit '\e[20l'; at 5 5; printf 'LNM off\nnext keeps column'
emit '\e[20h'; at 9 5; printf 'LNM on\nnext starts at column 1'
emit '\e[20l'; stty "$original_stty"
