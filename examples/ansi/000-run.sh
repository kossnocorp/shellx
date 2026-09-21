#!/usr/bin/env bash
set -euo pipefail
directory=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
if [[ ${1:-} == --help ]]; then
  printf 'Usage: %s [number-or-filename-prefix]\n' "$0"
  printf 'Runs demos in filename order. n advances; p goes back; r repeats; q quits.\n'
  printf 'At the prompt: Up increases delay; Down decreases it (0.25s steps).\n'
  exit 0
fi
if [[ ! -t 0 || ! -t 1 ]]; then
  printf 'Run this explorer directly in a terminal, without redirection.\n' >&2
  exit 1
fi
ANSI_DEMO_STATE_FILE=$(mktemp "${TMPDIR:-/tmp}/ansi-demo-delay.XXXXXXXX")
export ANSI_DEMO_STATE_FILE
trap 'rm -f -- "$ANSI_DEMO_STATE_FILE"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
printf '%s\n' "${ANSI_DEMO_DELAY_MS:-1000}" > "$ANSI_DEMO_STATE_FILE"
start=${1:-}
declare -a demos=()
for demo in "$directory"/[0-9][0-9][0-9]-*.sh; do
  [[ ${demo##*/} == 000-run.sh ]] || demos+=("$demo")
done
index=0
if [[ -n $start ]]; then
  while ((index < ${#demos[@]})); do
    name=${demos[index]##*/}
    [[ $name == "$start"* ]] && break
    index=$((index + 1))
  done
  if ((index == ${#demos[@]})); then printf 'No demo matches: %s\n' "$start" >&2; exit 1; fi
fi
while ((index < ${#demos[@]})); do
  demo=${demos[index]}
  printf '\033[0m\033[2J\033[H'
  status=0
  ANSI_RUNNER=1 bash "$demo" || status=$?
  case $status in
    0) index=$((index + 1)) ;;
    10) continue ;;
    20) exit 0 ;;
    30) if ((index > 0)); then index=$((index - 1)); fi ;;
    *) printf '\nDemo stopped (exit %s): %s\n' "$status" "${demo##*/}" >&2; exit "$status" ;;
  esac
done
printf '\nAll ANSI demos completed.\n'
