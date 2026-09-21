#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Control-string envelopes: SOS, PM, APC; terminate with ST' 'These have no universal visible action. Payloads should usually be ignored.'
printf 'Before SOS |'; emit '\eXignored SOS payload\e\\'; printf '| after\n'
printf 'Before PM  |'; emit '\e^ignored PM payload\e\\'; printf '| after\n'
printf 'Before APC |'; emit '\e_ignored APC payload\e\\'; printf '| after\n'
note 'DCS has specialized protocols (queries, Sixel, ReGIS); OSC handles titles, links, etc.'
note 'Some terminals without a string parser may expose payload text instead.'
