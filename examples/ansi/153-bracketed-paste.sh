#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Bracketed paste: CSI ?2004 h/l' 'Paste some text. Expect CSI 200~ before it and CSI 201~ afterward.'
emit '\e[?2004h'; events 10; emit '\e[?2004l'
