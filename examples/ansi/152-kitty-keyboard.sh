#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Kitty keyboard protocol: query, push, set, pop' 'Flag 1 disambiguates escape codes; flag 2 requests event types.'
report 'CSI ? u: query current flags' '\e[?u'
emit '\e[>1u'; defer $'\e[<1u'
note 'CSI >1u pushed flags=1; CSI =3;1u sets flags=3 on that stack level.'
emit '\e[=3;1u'; events
emit '\e[<1u'
# The explicit pop completed; do not pop twice during cleanup.
unset 'restores[${#restores[@]}-1]'
