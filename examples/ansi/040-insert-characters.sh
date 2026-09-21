#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Insert characters: CSI 4 @' 'Insert four blank cells at column 4; following text shifts right.'
printf 'ABCDEFGHIJKLMNO'; delay
emit '\e[4G\e[4@'; delay; printf '1234\n'
