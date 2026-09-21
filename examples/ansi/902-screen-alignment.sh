#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'Screen alignment test: ESC # 8' 'The screen should fill with E characters.'
delay 1; emit '\e#8'
