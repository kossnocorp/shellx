#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Hyperlinks: OSC 8 ; parameters ; URI ST' 'Hover/click using your terminal modifier. The label and URL can differ.'
printf '\033]8;;https://example.com\033\\\033[4;34mOpen example.com\033[0m\033]8;;\033\\\n'
printf '\033]8;id=shared;https://example.com\033\\First fragment\033]8;;\033\\ ordinary text '
printf '\033]8;id=shared;https://example.com\033\\second fragment (same link id)\033]8;;\033\\\n'
