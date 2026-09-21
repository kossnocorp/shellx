#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Full terminal reset: ESC c (RIS)' 'Last demo: may clear the screen, reset modes, and reset terminal customizations.'
delay 2; emit '\ec'; delay
note 'Full reset sent. The terminal should be back in its initial state.'
