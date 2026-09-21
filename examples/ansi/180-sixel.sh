#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Sixel raster graphics: DCS parameters q ... ST' 'Expected: a small red/blue striped rectangle. Unsupported terminals may show nothing.'
at 5 1
# Raster dimensions 120x60; each ~ is a vertical group of six painted pixels.
emit '\eP0;1;0q"1;1;120;60#0;2;100;20;10#1;2;10;40;100'
for ((i=0; i<10; i++)); do printf '#%s!120~-' "$((i%2))"; done
emit '\e\\'
