#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Desktop notifications: OSC 9 / 777 / 99' 'Some terminals notify only when unfocused; OS notification settings also apply.'
note 'OSC 9;message'; emit '\e]9;ANSI explorer: OSC 9 notification\e\\'; delay 1
note 'OSC 777;notify;title;body'; emit '\e]777;notify;ANSI explorer;OSC 777 notification\e\\'; delay 1
note 'OSC 99;i=ansi-demo;title (Kitty-style simple notification)'
emit '\e]99;i=ansi-demo;ANSI explorer: OSC 99 notification\e\\'
