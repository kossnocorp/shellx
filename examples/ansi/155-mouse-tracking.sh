#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Mouse tracking modes with SGR encoding (?1006h)' 'Click and drag here. SGR events: CSI <button;x;y M (press/move), m (release).'
emit '\e[?1006h'
for mode in 9 1000 1002 1003; do
  case $mode in
    9) note 'Mode 9: X10, presses only' ;;
    1000) note 'Mode 1000: presses and releases' ;;
    1002) note 'Mode 1002: motion while a button is held' ;;
    1003) note 'Mode 1003: all motion, even without buttons' ;;
  esac
  printf '\033[?%sh' "$mode"; events 5; printf '\033[?%sl' "$mode"
done
emit '\e[?1006l'
