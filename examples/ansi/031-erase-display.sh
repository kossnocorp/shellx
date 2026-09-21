#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
for mode in 0 1 2; do
  emit '\e[2J\e[H'
  heading "Erase display: CSI $mode J" 'Watch the filled rows; cursor starts at row 9, column 15.'
  rows; at 9 15; delay 1
  printf '\033[%sJ@' "$mode"; delay 1.5
done
at 17 1; note 'CSI 2 J erased the display; @ shows that the cursor stayed in place.'
