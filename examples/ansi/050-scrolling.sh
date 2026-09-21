#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Scroll up/down: CSI 2 S / CSI 2 T' 'Only the region 5–14 moves. The cursor does not move with the content.'
rows; emit '\e[5;14r'; at 10 45; printf '@'; delay
emit '\e[2S'; delay 1.5
emit '\e[2T'; delay
