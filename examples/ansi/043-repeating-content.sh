#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Repeat preceding graphic character: REP / CSI n b' 'One printed # followed by CSI 30 b produces 31 copies.'
printf '#\033[30b\n'
printf '\033[36m=\033[40b\033[0m\n'
note 'This repeats a character, not an arbitrary string.'
