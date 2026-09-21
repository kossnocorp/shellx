#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Normal/application cursor and keypad modes' 'Press arrows, Home/End, Insert/Delete, Page Up/Down, F1–F4, and keypad keys.'
defer $'\e[?1l\e>'
note 'Normal cursor (?1l) and numeric keypad (ESC >):'
emit '\e[?1l\e>'; events
note 'Application cursor (?1h) and application keypad (ESC =):'
emit '\e[?1h\e='; events
emit '\e[?1l\e>'
note 'Typical normal arrows: CSI A/B/C/D; application: ESC O A/B/C/D.'
note 'Insert/Delete: CSI 2~/3~; PgUp/PgDn: CSI 5~/6~; F1–F4: ESC O P/Q/R/S.'
