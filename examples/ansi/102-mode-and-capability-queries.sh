#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Mode and capability queries' 'Mode status: 0 unknown, 1 set, 2 reset, 3 permanently set, 4 permanently reset.'
report 'CSI 4 $ p: insert mode' '\e[4$p'
report 'CSI ? 7 $ p: automatic wrapping' '\e[?7$p'
report 'CSI ? 2026 $ p: synchronized-output support' '\e[?2026$p'
report 'DCS $ q m ST: DECRQSS, current SGR' '\eP$qm\e\\'
report 'DCS $ q r ST: DECRQSS, scrolling margins' '\eP$qr\e\\'
report 'DCS + q 544e;436f;524742 ST: XTGETTCAP (TN, Co, RGB)' '\eP+q544e;436f;524742\e\\'
