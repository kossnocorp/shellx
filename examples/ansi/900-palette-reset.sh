#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Reset whole palette: OSC 104 ST' 'Resets all palette entries to configured defaults, including prior customizations.'
note 'A red sample before palette reset:'; sample 31 'Red palette entry'
emit '\e]104\e\\'
sample 31 'Red palette entry after reset'
