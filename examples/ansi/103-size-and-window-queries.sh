#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Window / cell / screen geometry queries'
report 'CSI 11 t: window state' '\e[11t'
report 'CSI 13 t: window position' '\e[13t'
report 'CSI 14 t: text area pixels' '\e[14t'
report 'CSI 16 t: cell pixels' '\e[16t'
report 'CSI 18 t: text area rows/columns' '\e[18t'
report 'CSI 19 t: screen rows/columns' '\e[19t'
