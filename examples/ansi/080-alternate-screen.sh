#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Alternate screen: CSI ? 1049 h/l' 'This normal-screen text should return after leaving the alternate screen.'
delay 1
screen
heading 'You are now on the alternate screen' 'The original screen and cursor return when you advance or exit.'
rows
