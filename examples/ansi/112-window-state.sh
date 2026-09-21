#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Window state operations' 'Iconify/deiconify, lower/raise, maximize/restore, fullscreen/leave, refresh.' 'The window may briefly disappear. This demo restores a visible, non-fullscreen window.'
defer $'\e[1t\e[5t\e[9;0t\e[10;0t'
note 'Iconify for one second'; emit '\e[2t'; delay 1; emit '\e[1t'
note 'Lower for one second'; emit '\e[6t'; delay 1; emit '\e[5t'
note 'Maximize for one second'; emit '\e[9;1t'; delay 1; emit '\e[9;0t'
note 'Fullscreen for one second'; emit '\e[10;1t'; delay 1; emit '\e[10;0t'
emit '\e[7t'; note 'Refresh requested.'
