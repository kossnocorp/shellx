#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading '16 foreground and background colors' 'SGR 30–37 / 90–97 foreground; 40–47 / 100–107 background.'
for ((i=0; i<8; i++)); do
  printf '\033[%sm FG %s \033[%sm bright %s \033[0m  ' "$((30+i))" "$((30+i))" "$((90+i))" "$((90+i))"
  printf '\033[30;%sm BG %s \033[30;%sm bright %s \033[0m\n' "$((40+i))" "$((40+i))" "$((100+i))" "$((100+i))"
done
printf '\033[31;44mRed on blue\033[39m default FG\033[49m default BG\033[0m\n'
note 'SGR 39 and 49 restore defaults independently.'
