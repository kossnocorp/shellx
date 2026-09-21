#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Legacy printer controls: CSI 0/4/5 i' 'Requests a screen print, then sends a short line through printer-controller mode.' 'Usually ignored; a configured terminal printer may actually print or spool this.'
defer $'\e[4i'
emit '\e[0i'; delay
emit '\e[5i'; printf 'ANSI explorer printer-controller sample\r\n'; emit '\e[4i'
note 'Printer-controller mode disabled.'
