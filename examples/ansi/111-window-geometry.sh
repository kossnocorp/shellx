#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Window move/resize: CSI 3/4/8 ... t' 'May be ignored. Original character size and queried pixel geometry are restored.'
read -r height width < <(stty size)
defer "${CSI}8;${height};${width}t"
query '\e[13t'
if [[ $REPLY =~ $'\e\x5b'3\;([0-9]+)\;([0-9]+)t ]]; then
  defer "${CSI}3;${BASH_REMATCH[1]};${BASH_REMATCH[2]}t"
fi
query '\e[14t'
if [[ $REPLY =~ $'\e\x5b'4\;([0-9]+)\;([0-9]+)t ]]; then
  defer "${CSI}4;${BASH_REMATCH[1]};${BASH_REMATCH[2]}t"
fi
note 'Move window to x=80, y=80'; emit '\e[3;80;80t'; delay 1
note 'Resize to 600 pixels high by 900 wide'; emit '\e[4;600;900t'; delay 1
note 'Resize to 28 rows by 90 columns'; emit '\e[8;28;90t'
