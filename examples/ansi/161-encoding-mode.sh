#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Character encoding selection: ESC % @ / ESC % G' 'Many modern terminals stay in UTF-8 regardless. This demo restores UTF-8.'
defer $'\e%G'
emit '\e%@'; printf 'Legacy mode: ASCII is still readable.\n'
emit '\e%G'; printf 'UTF-8 mode: café └ │ 世界\n'
