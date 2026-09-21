#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Focus reporting: CSI ?1004 h/l' 'Switch away and back. CSI O means focus lost; CSI I means focus gained.'
emit '\e[?1004h'; events 10; emit '\e[?1004l'
