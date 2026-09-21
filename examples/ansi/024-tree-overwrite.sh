#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'The original tree example' 'CSI 1 A + CR overwrites the previous connector; CSI 1 B + CR returns.'
printf '• Fetching repositories\n'
for repo in alpha beta gamma delta; do
  if [[ $repo != alpha ]]; then printf '\033[1A\r  │\033[1B\r'; fi
  printf '  └ %s\n' "$repo"
  delay 0.8
done
