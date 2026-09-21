#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Insert mode: CSI 4 h / CSI 4 l' 'Type 123 in the middle: existing characters shift rather than being overwritten.'
printf 'ABCDEFGHIJKLMNO'; delay
emit '\e[4G\e[4h'; printf '123'; emit '\e[4l'; printf '\n'
