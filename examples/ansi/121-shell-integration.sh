#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Shell integration: OSC 7 and OSC 133 A/B/C/D' 'Look for prompt marks/navigation in supporting terminals. This is a simulated command.'
cwd=$PWD
cwd=${cwd//%/%25}; cwd=${cwd// /%20}; cwd=${cwd//#/%23}; cwd=${cwd//\?/%3F}
printf '\033]7;file://localhost%s\033\\' "$cwd"
emit '\e]133;A\e\\'; printf 'demo$ '; emit '\e]133;B\e\\'
printf 'echo hello\n'; emit '\e]133;C\e\\'; printf 'hello\n'
emit '\e]133;D;0\e\\'
note 'OSC 7 advertised the current directory; OSC 133 marked a successful command.'
