#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
source "$(dirname -- "${BASH_SOURCE[0]}")/_image.sh"
screen
heading 'iTerm2 image protocol: OSC 1337;File=...:base64 ST' 'Expected: orange/blue checkerboard, scaled to 16 columns by 8 rows.'
at 5 1
printf '\033]1337;File=name=Y2hlY2tlci5wbmc=;inline=1;width=16;height=8;preserveAspectRatio=0:%s\033\\' "$image_png"
