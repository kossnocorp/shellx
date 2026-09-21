#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading '256-color palette: SGR 38;5;n and 48;5;n' 'Palette entries can be customized by the terminal.'
for ((i=0; i<256; i++)); do
  printf '\033[48;5;%sm\033[38;5;%sm%4d\033[0m' "$i" "$((i<8 || (i>=16 && i<232 && (i-16)%36<18) || (i>=232 && i<244) ? 15 : 0))" "$i"
  if (((i+1)%16==0)); then printf '\n'; fi
done
