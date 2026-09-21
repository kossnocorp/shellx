#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Device attributes and terminal version'
report 'CSI c: primary attributes' '\e[c'
report 'CSI > c: secondary attributes' '\e[>c'
report 'CSI = c: tertiary attributes' '\e[=c'
report 'CSI > 0 q: XTVERSION' '\e[>0q'
