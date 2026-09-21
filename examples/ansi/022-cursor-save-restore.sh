#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Save and restore cursor' 'Save slots are not a stack. DEC ESC 7/8 may also save attributes.'
at 5 1; printf 'DEC: '; emit '\e7'
at 8 20; printf 'temporary location'; delay
emit '\e8'; printf 'returned with ESC 8'
at 11 1; printf 'CSI: '; emit '\e[s'
at 14 20; printf 'another location'; delay
emit '\e[u'; printf 'returned with CSI u'
