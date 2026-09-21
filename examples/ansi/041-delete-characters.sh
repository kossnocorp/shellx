#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Delete characters: CSI 4 P' 'Delete four cells at column 4; following text shifts left.'
printf 'ABCDEFGHIJKLMNO'; delay
emit '\e[4G\e[4P'; printf '\n'
