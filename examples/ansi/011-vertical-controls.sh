#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'VT and FF; LF with output translation disabled' 'Modern terminals commonly treat vertical tab and form feed like line feed.'
stty -onlcr
at 5 5; printf 'LF\nnext'; delay
at 8 5; printf 'VT\vnext'; delay
at 11 5; printf 'FF\fnext'
stty "$original_stty"
