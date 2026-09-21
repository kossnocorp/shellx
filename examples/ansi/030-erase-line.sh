#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Erase line: CSI 0 K, 1 K, 2 K' 'Erasure blanks cells, without shifting text or moving the cursor.'
for mode in 0 1 2; do
  note "Mode $mode: before -> after (cursor at column 11)"
  printf '0123456789ABCDEFGHIJ'; delay
  printf '\033[11G\033[%sK|\n' "$mode"
done
note 'Typical progress line: CR + CSI 2 K'
for progress in 'Starting a long operation...' '50%' 'Done'; do
  printf '\r\033[2K%s' "$progress"; delay
done
printf '\n'
