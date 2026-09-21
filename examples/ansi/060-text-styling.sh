#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'SGR text attributes: CSI parameters m' 'Each sample resets with SGR 0. Concealed text still exists in the output.'
for entry in '0 normal' '1 bold' '2 dim' '3 italic' '4 underline' '5 slow-blink' '6 rapid-blink' '7 reverse' '8 concealed' '9 strikethrough'; do
  sample "${entry%% *}" "${entry#* } — Sample text 0123456789"
done
note 'Attribute-specific resets: styled -> reset (foreground remains cyan)'
for pair in '1:22' '2:22' '3:23' '4:24' '5:25' '6:25' '7:27' '8:28' '9:29'; do
  printf '\033[0m%-12s \033[36;%smstyled\033[%sm reset\033[0m\n' "$pair" "${pair%:*}" "${pair#*:}"
done
sample '1;2;3;4;34' 'Combined bold + dim + italic + underline + blue'
