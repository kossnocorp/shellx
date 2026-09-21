#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Status and cursor queries' 'Replies are shown escaped; no reply times out. Do not type during queries.'
report 'CSI 5 n: status (normally CSI 0 n)' '\e[5n'
report 'CSI 6 n: cursor position (CSI row;col R)' '\e[6n'
report 'CSI ? 6 n: extended cursor position' '\e[?6n'
