#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Window/icon titles: OSC 0, 1, 2; CSI 22/23;0 t' 'Watch the terminal title/tab. Title stack saves/restores where supported.'
emit '\e[22;0t'; defer $'\e[23;0t'
emit '\e]0;ANSI explorer — icon AND title\e\\'; delay 1
emit '\e]1;ANSI explorer — icon name\e\\'; delay 1
emit '\e]2;ANSI explorer — window title\e\\'; delay 1
note 'OSC can also end in BEL rather than ST:'
emit '\e]2;ANSI explorer — BEL terminated\a'
