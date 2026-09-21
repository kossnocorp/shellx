#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Truecolor: SGR 38;2;r;g;b and 48;2;r;g;b' 'Smooth background gradient; also compare semicolon and colon forms.'
for ((i=0; i<64; i++)); do printf '\033[48;2;%s;%s;180m ' "$((i*4))" "$((255-i*4))"; done
emit '\e[0m\n'
sample '38;2;255;120;40' 'Semicolon RGB foreground'
sample '38:2::255:120:40' 'Colon RGB foreground (empty color-space field)'
sample '48:2::30:80:130' 'Colon RGB background'
