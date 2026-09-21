#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Protected content: DECSCA + selective erase' 'CSI 1 " q protects; CSI 0 " q unprotects; CSI ? 2 K/J selectively erase.'
defer $'\e[0"q'
at 5 1; emit '\e[1"q'; printf 'PROTECTED'; emit '\e[0"q'; printf ' ordinary'; delay
emit '\e[?2K'; delay
at 8 1; emit '\e[1"q'; printf 'ALSO PROTECTED'; emit '\e[0"q'; printf ' ordinary'; delay
emit '\e[?2J'
at 12 1; note 'Only protected text should remain above (if supported).'
