#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Underline styles and independent underline colors' 'Colon subparameters and SGR 58/59 are terminal-dependent.'
for entry in '0 none' '1 single' '2 double' '3 curly' '4 dotted' '5 dashed'; do
  sample "4:${entry%% *}" "${entry#* } underline"
done
sample '21' 'SGR 21: double underline (historically ambiguous)'
sample '4;58;5;196' 'Indexed red underline'
sample '4:3;58;2;255;100;0' 'RGB orange curly underline'
printf '\033[4;58;5;196mRed underline\033[59m default underline color\033[24m off\033[0m\n'
