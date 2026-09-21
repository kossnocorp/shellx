#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Whole-screen reverse video: CSI ? 5 h/l' 'This affects the screen, unlike SGR 7, which affects following text.'
defer $'\e[?5l'
note 'Normal screen'; delay 1
emit '\e[?5h'; note 'Reverse screen'; delay 2
emit '\e[?5l'; note 'Normal again'
