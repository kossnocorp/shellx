#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'Clipboard: OSC 52;c;BASE64 ST; query with ?' 'Writes "ANSI explorer clipboard demo" to the clipboard.' 'If the terminal permits a clipboard query, its original value is restored on exit.'
query '\e]52;c;?\e\\'
if [[ $REPLY == "${OSC}52;c;"* && ( $REPLY == *"$ST" || $REPLY == *$'\a' ) && $REPLY != "${OSC}52;c;?"* ]]; then
  defer "$REPLY"
  note 'Original clipboard captured for restoration (contents not displayed).'
else
  note 'Clipboard query unavailable: the demo text will remain in the clipboard if writing succeeds.'
fi
emit '\e]52;c;QU5TSSBleHBsb3JlciBjbGlwYm9hcmQgZGVtbw==\e\\'
note 'Try pasting into another application before advancing.'
note 'Selection parameter p targets the primary selection on supporting systems.'
