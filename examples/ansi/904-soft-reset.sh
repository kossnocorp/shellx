#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Soft terminal reset: CSI ! p (DECSTR)' 'Resets several modes without intentionally clearing the display.'
emit '\e[31;1m'; printf 'Bold red text before reset'; delay
emit '\e[!p'; printf '\nText after soft reset (normally default attributes).\n'
