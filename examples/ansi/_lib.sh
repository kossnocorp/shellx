#!/usr/bin/env bash
# Shared terminal I/O and cleanup. Source from a numbered demo, not directly.
set -euo pipefail

if [[ ! -t 0 || ! -t 1 ]]; then
  printf 'ANSI demos need a terminal on stdin and stdout. Run directly, without a pipe.\n' >&2
  exit 1
fi

ESC=$'\e'
CSI=$'\e['
OSC=$'\e]'
ST=$'\e\\'
original_stty=$(stty -g)
alternate=false
heading_started=false
demo_path="examples/ansi/${BASH_SOURCE[1]##*/}"
delay_ms=${ANSI_DEMO_DELAY_MS:-1000}
if [[ -n ${ANSI_DEMO_STATE_FILE:-} ]]; then
  IFS= read -r delay_ms < "$ANSI_DEMO_STATE_FILE"
fi
if [[ ! $delay_ms =~ ^[0-9]{1,5}$ ]]; then
  printf 'ANSI_DEMO_DELAY_MS must be an integer from 100 to 10000.\n' >&2
  exit 1
fi
delay_ms=$((10#$delay_ms))
if ((delay_ms < 100 || delay_ms > 10000)); then
  printf 'ANSI_DEMO_DELAY_MS must be an integer from 100 to 10000.\n' >&2
  exit 1
fi
declare -a restores=()

emit() { printf '%b' "$1"; }
note() { printf '%s\n' "$*"; }
section() {
  # Optional second argument highlights only the sequence; all text stays bold.
  printf '\n\033[1m%s' "$1"
  if (($# > 1)); then
    printf '\033[34m%s\033[39m%s' "$2" "${3:-}"
  fi
  printf '\033[0m\n'
}
delay() {
  # Preserve relative timings: the original default step was 0.6 seconds.
  local seconds=${1:-0.6} whole fraction milliseconds duration
  whole=${seconds%%.*}
  fraction=0
  [[ $seconds != *.* ]] || fraction=${seconds#*.}
  fraction=${fraction}000
  milliseconds=$(((10#$whole * 1000 + 10#${fraction:0:3}) * delay_ms / 600))
  printf -v duration '%d.%03d' "$((milliseconds / 1000))" "$((milliseconds % 1000))"
  sleep "$duration"
}
defer() { restores+=("$1"); }
heading() {
  if ! $heading_started; then
    emit '\e[2J\e[H'
    heading_started=true
  fi
  printf '\n\033[1;97;44m %s \033[0m\n' "$1"
  shift
  if (($#)); then printf '%s\n' "$@"; fi
  printf '\n'
}

# Consume a complete arrow key rather than treating its ESC byte as "next".
# Support both normal (CSI) and application (SS3) cursor-key encodings.
navigation_key() {
  local char sequence='' count
  REPLY=''
  while ((status == 0)); do
    if IFS= read -r -s -n 1 -t 0.1 REPLY; then break; fi
  done
  ((status == 0)) || return 1
  [[ $REPLY == "$ESC" ]] || return 0
  REPLY=escape
  IFS= read -r -s -n 1 -t 0.1 char || return 0
  [[ $char == '[' || $char == O ]] || return 0
  sequence=$char
  for ((count=0; count<16; count++)); do
    IFS= read -r -s -n 1 -t 0.1 char || break
    sequence+=$char
    [[ $char != [[:alpha:]~] ]] || break
  done
  case $sequence in
    '[A'|OA) REPLY=up ;;
    '[B'|OB) REPLY=down ;;
  esac
}
label() { printf '\033[0;1m%-24s\033[0m ' "$1"; }
screen() {
  alternate=true
  emit '\e[?1049h\e[2J\e[H'
}
at() { printf '\033[%s;%sH' "$1" "$2"; }
sample() {
  label "SGR $1"
  printf '\033[%sm%s\033[0m\n' "$1" "${2:-The quick brown fox 0123456789}"
}
rows() {
  local n
  for ((n=5; n<=14; n++)); do
    at "$n" 1
    printf '%02d | abcdefghijklmnopqrstuvwxyz | 0123456789' "$n"
  done
}

# Read replies from the actual input terminal, with echo and canonical mode off.
# The first-byte timeout is one second; subsequent bytes have a short deadline.
# Even a continuous stream cannot keep the query running indefinitely.
query() {
  local LC_ALL=C byte timeout=1 deadline=$((SECONDS + 2))
  REPLY=''
  stty -echo -icanon min 0 time 0
  printf '%b' "$1"
  while ((SECONDS < deadline)) && IFS= read -r -s -N 1 -t "$timeout" byte; do
    REPLY+=$byte
    timeout=0.1
  done
  stty "$original_stty"
}
report() {
  section "$1"
  query "$2"
  if [[ -n $REPLY ]]; then printf '  Reply: %q\n' "$REPLY";
  else note '  No reply (unsupported, disabled, or intercepted by a multiplexer).'; fi
}

# Replay an OSC color reply on exit to restore the exact original value.
save_color() {
  local code=$1 fallback=$2
  query "${OSC}${code};?${ST}"
  if [[ $REPLY == "${OSC}${code};"* && ( $REPLY == *"$ST" || $REPLY == *$'\a' ) ]]; then
    defer "$REPLY"
  else
    defer "$fallback"
    note "Color $code could not be queried; exit will reset it to the terminal default."
  fi
}

events() {
  local LC_ALL=C duration=${ANSI_DEMO_EVENT_SECONDS:-${1:-8}} byte deadline
  deadline=$((SECONDS + duration))
  note "Interact now for ${duration}s. Received bytes appear in Bash escaped form."
  stty -echo -icanon min 0 time 0
  while ((SECONDS < deadline)); do
    if IFS= read -r -s -N 1 -t 0.1 byte; then printf '%q ' "$byte"; fi
  done
  stty "$original_stty"
  printf '\n'
}

cleanup() {
  local status=$? key='' index
  set +e
  trap - EXIT
  # Navigation is still part of cleanup: an interrupt must reach the restores.
  trap 'status=130' INT
  trap 'status=143' TERM
  trap 'status=129' HUP
  # Resetting origin mode and margins homes the cursor. Keep the output position
  # so the navigation prompt does not overwrite the title at the top of the screen.
  emit '\e7'
  # Stop input reports before waiting for navigation. Keep the visible screen.
  emit '\e[?9l\e[?1000l\e[?1002l\e[?1003l\e[?1004l\e[?1005l\e[?1006l\e[?1015l\e[?1016l\e[?2004l'
  emit '\e[?2026l\e[?6l\e[?69l\e[r\e[4l\e[20l\e[?7h\e[0m\e[?25h'
  emit '\e8\e[0m'
  stty "$original_stty"
  if ((status == 0)) && [[ ${ANSI_DEMO_NO_PAUSE:-0} != 1 ]]; then
    if $alternate; then
      at "$(($(tput lines 2>/dev/null || printf 24) - 1))" 1
    else
      # Allocate both footer rows before drawing, including at the screen bottom.
      printf '\n\n\n\033[2F'
    fi
    while ((status == 0)); do
      printf '\r\033[2K\033[1;35m[n: next/exit | p: previous | r: repeat | q: quit | ↑/↓: %d ms]\033[0m' "$delay_ms"
      printf '\n\r\033[2K\033[2m%s\033[0m\r\033[1A' "$demo_path"
      navigation_key || break
      key=$REPLY
      case $key in
        up) delay_ms=$((delay_ms + 250)); ((delay_ms <= 10000)) || delay_ms=10000 ;;
        down) delay_ms=$((delay_ms - 250)); ((delay_ms >= 100)) || delay_ms=100 ;;
        n|N|q|Q) break ;;
        p|P|r|R) if [[ ${ANSI_RUNNER:-0} == 1 ]]; then break; fi ;;
        *) continue ;;
      esac
    done
    printf '\033[1B\r\n'
  fi
  if [[ -n ${ANSI_DEMO_STATE_FILE:-} ]]; then
    printf '%s\n' "$delay_ms" > "$ANSI_DEMO_STATE_FILE"
  fi
  for ((index=${#restores[@]}-1; index>=0; index--)); do
    printf '%s' "${restores[index]}"
  done
  if $alternate; then emit '\e[?1049l'; fi
  stty "$original_stty"
  if [[ ${ANSI_RUNNER:-0} == 1 && $status == 0 ]]; then
    case $key in r|R) status=10 ;; q|Q) status=20 ;; p|P) status=30 ;; esac
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'exit 129' HUP
