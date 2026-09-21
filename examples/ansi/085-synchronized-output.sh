#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Synchronized output: CSI ? 2026 h/l' 'Top: incremental updates. Bottom: one batch if supported (terminal timeouts vary).'
at 5 1; printf 'Unbatched: '
for ((i=0; i<20; i++)); do printf '#'; delay 0.03; done
at 8 1; printf 'Batched:   '
emit '\e[?2026h'
for ((i=0; i<20; i++)); do printf '#'; delay 0.03; done
emit '\e[?2026l'
