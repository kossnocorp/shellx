#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Vendor shell integration: OSC 633 and OSC 1337' 'OSC 633 prompt marks (VS Code-style); OSC 1337 CurrentDir (iTerm2-style).'
emit '\e]633;A\e\\'; printf 'vendor-demo$ '; emit '\e]633;B\e\\'
printf 'echo hello\n'; emit '\e]633;C\e\\'; printf 'hello\n'; emit '\e]633;D;0\e\\'
printf '\033]1337;CurrentDir=%s\033\\' "$PWD"
