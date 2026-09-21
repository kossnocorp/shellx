# Terminal escape-sequence explorer

Run the guided tour directly in a terminal:

```bash
./examples/ansi/000-run.sh
```

- **n** advances after you have inspected a demo. Unrecognized keys, including
  Space and Enter, do not advance.
- **r** repeats the current demo.
- **p** goes to the previous demo, including earlier than the selected starting
  demo. At the first demo, it repeats that demo.
- **q** quits the tour.
- **↑ / ↓** at the navigation prompt increases/decreases the delay in 0.25-second
  steps (0.10–10 seconds). The prompt updates immediately; the setting applies to
  repeats and subsequent demos. Arrow keys do not advance the tour.
- **Ctrl-C** interrupts and runs terminal cleanup.

Each run/repeat starts with a cleared display. The navigation prompt shows the current
base delay in milliseconds (default **1000 ms**); longer pauses and short animations scale proportionally.
Query timeouts and interactive input windows retain their original durations.
To choose a starting delay, use `ANSI_DEMO_DELAY_MS=2000 ./examples/ansi/000-run.sh`.

Start at a number or filename prefix, or run a standalone demo:

```bash
./examples/ansi/000-run.sh 060
./examples/ansi/000-run.sh 150-keyboard
./examples/ansi/043-repeating-content.sh
```

Each standalone demo also waits for **n** or **q** before restoring its terminal
state and exiting. No `shx` binary, build step, downloads, or image files are needed.
Requirements: Bash, `stty`, `tput`, and `sleep`. An 80-column, 24-row terminal or
larger works best. Avoid resizing during coordinate-based demos.

## What to expect

The collection covers the ANSI/ECMA-48, DEC, xterm, and modern extension families
in the accompanying catalog below. It is not a claim that every vendor's terminal
protocol, parameter combination, or historical ANSI control function is included.

Common movement, erasure, and styling should work in most terminal emulators.
Other sequences may be ignored, and multiplexers such as tmux can filter them.
Try a direct terminal session to distinguish emulator support from multiplexer
behavior. Unsupported features may produce no visible change; old or incomplete
parsers may instead display fragments of a sequence.

Input demos have timed interaction windows, followed by the normal navigation
prompt. Query demos have bounded timeouts and display replies using Bash's escaped
representation. Do not type during a query: terminal responses and keystrokes
share the same input stream. The scripts do not execute the received text.

Many screen-coordinate demos use the alternate screen and restore the original
screen on exit. Cleanup also restores the saved `stty` settings and returns
wrapping, scrolling margins, cursor visibility, and input-reporting modes to
ordinary shell defaults. Cleanup is not a complete snapshot of arbitrary terminal
state; for example, custom tab stops return to conventional eight-column stops,
and cursor shape returns to the terminal default.

### Demos with effects beyond their printed text

- `110` changes the title and uses the terminal title stack to restore it.
- `111` moves/resizes the window; queried geometry is restored where supported.
- `112` briefly iconifies, lowers, maximizes, and fullscreen-displays the window,
  then returns it to a visible, non-fullscreen state.
- `121`/`122` create shell-integration marks that can remain in terminal history.
- `130`–`132` and `071` query colors before changing them and replay the saved
  values on exit. If querying fails, they reset to configured defaults instead.
- `140` writes a demonstration clipboard value. It restores the old value only
  if the terminal permits reading it. Original clipboard contents are not printed.
- `141` sends desktop notifications; `142` changes a taskbar/tab progress indicator.
- `191` can print/spool content if terminal printer support is configured.
- `900` resets the entire palette, `901` can resize/clear the screen, `903` clears
  scrollback irreversibly, and `905` performs a full terminal reset. These are at
  the end so they do not interrupt earlier exploration.

These effects are part of the demonstrations. A separate terminal window is a
convenient place to explore them. Interrupt cleanup cannot undo erased scrollback,
delivered notifications, printing, or a full reset.

## Sequence notation

- `ESC` is byte `0x1b`, written `\e` / `\033` in these Bash scripts.
- `CSI` is `ESC [`; `OSC` is `ESC ]`; `DCS` is `ESC P`.
- `ST` is `ESC \`, the string terminator. OSC also commonly accepts BEL (`\a`).
- `APC` is `ESC _`, `PM` is `ESC ^`, and `SOS` is `ESC X`.
- `SP` means a literal space. Coordinates are normally 1-based.
- `h/l` means separate enable/disable sequences. Counts and coordinates are
  representative values, not exhaustive enumerations of possible parameters.
- The scripts use 7-bit ESC-prefixed forms, not raw 8-bit C1 bytes, which can
  conflict with UTF-8 decoding.

Read each short script to see the exact emitted bytes. `emit` uses `printf '%b'`
for escape literals; ordinary explanatory text uses `printf '%s'`.

## Coverage index

### Controls, cursor, erasure, editing, scrolling

| Demo | Sequences / features |
| --- | --- |
| [010-control-characters](010-control-characters.sh) | CR, LF, BS, HT, BEL, CAN, SUB |
| [011-vertical-controls](011-vertical-controls.sh) | VT, FF, LF without OS newline translation |
| [020-cursor-relative](020-cursor-relative.sh) | CUU/CUD/CUF/CUB `CSI n A/B/C/D`; CNL/CPL `E/F`; HPR/VPR `a/e` |
| [021-cursor-absolute](021-cursor-absolute.sh) | CUP/HVP `H/f`; CHA `G`; HPA backtick; VPA `d`; default `CSI H` |
| [022-cursor-save-restore](022-cursor-save-restore.sh) | DEC `ESC 7/8`; common `CSI s/u` |
| [023-index-and-reverse-index](023-index-and-reverse-index.sh) | IND `ESC D`; NEL `ESC E`; RI `ESC M` |
| [024-tree-overwrite](024-tree-overwrite.sh) | Application: change the previous `└` into `│` as children arrive |
| [030-erase-line](030-erase-line.sh) | EL `CSI 0/1/2 K`; CR + erase progress-line pattern |
| [031-erase-display](031-erase-display.sh) | ED `CSI 0/1/2 J` |
| [033-erase-characters](033-erase-characters.sh) | ECH `CSI n X` |
| [034-protected-content](034-protected-content.sh) | DECSCA `CSI n " q`; selective EL/ED `CSI ? n K/J` |
| [040-insert-characters](040-insert-characters.sh) | ICH `CSI n @` |
| [041-delete-characters](041-delete-characters.sh) | DCH `CSI n P` |
| [042-insert-delete-lines](042-insert-delete-lines.sh) | IL/DL `CSI n L/M` |
| [043-repeating-content](043-repeating-content.sh) | REP `CSI n b` |
| [044-insert-mode](044-insert-mode.sh) | IRM `CSI 4 h/l` |
| [050-scrolling](050-scrolling.sh) | SU/SD `CSI n S/T` |
| [051-margins-and-origin](051-margins-and-origin.sh) | DECSTBM `CSI top;bottom r`, reset `CSI r`; DECOM `?6h/l` |
| [052-horizontal-margins](052-horizontal-margins.sh) | DECLRMM `?69h/l`; DECSLRM `CSI left;right s` |

### Styling and display modes

| Demo | Sequences / features |
| --- | --- |
| [060-text-styling](060-text-styling.sh) | SGR 0–9; individual resets 22–29; combined attributes |
| [061-basic-colors](061-basic-colors.sh) | SGR 30–37, 40–47, 90–97, 100–107; defaults 39/49 |
| [062-indexed-colors](062-indexed-colors.sh) | SGR `38;5;n` / `48;5;n`; all 256 indices |
| [063-truecolor](063-truecolor.sh) | SGR `38/48;2;r;g;b`; colon color-space forms |
| [064-underline-styles](064-underline-styles.sh) | SGR `4:0`–`4:5`, 21/24, indexed/RGB 58, reset 59 |
| [065-uncommon-styles](065-uncommon-styles.sh) | Fonts 10–19; Fraktur 20/23; proportional 26/50; frame/circle 51/52/54; overline 53/55; ideogram 60–65; super/subscript 73–75 |
| [070-cursor-appearance](070-cursor-appearance.sh) | Show/hide `?25h/l`; blink `?12h/l`; DECSCUSR `CSI 0–6 SP q` |
| [071-cursor-color](071-cursor-color.sh) | OSC 12 set/query; OSC 112 reset |
| [080-alternate-screen](080-alternate-screen.sh) | `?1049h/l` |
| [081-legacy-screen-buffers](081-legacy-screen-buffers.sh) | `?47h/l`, `?1047h/l`, cursor save/restore `?1048h/l` |
| [082-wrapping](082-wrapping.sh) | DECAWM `?7h/l` |
| [083-screen-reverse](083-screen-reverse.sh) | DECSCNM `?5h/l` |
| [084-newline-mode](084-newline-mode.sh) | LNM `CSI 20 h/l` |
| [085-synchronized-output](085-synchronized-output.sh) | `?2026h/l` |
| [090-tab-stops](090-tab-stops.sh) | HTS `ESC H`; TBC `CSI 0/3 g`; CHT/CBT `CSI n I/Z` |

### Queries, window control, metadata, colors, desktop integration

| Demo | Sequences / features |
| --- | --- |
| [100-status-queries](100-status-queries.sh) | DSR `CSI 5 n`; CPR request `CSI 6 n`; DEC `CSI ? 6 n`; escaped replies |
| [101-device-queries](101-device-queries.sh) | DA1/DA2/DA3 `CSI c`, `CSI > c`, `CSI = c`; XTVERSION `CSI > 0 q` |
| [102-mode-and-capability-queries](102-mode-and-capability-queries.sh) | DECRQM standard/private `CSI [ ? ] mode $ p`, reports `$y`; DECRQSS `DCS $ q`; XTGETTCAP `DCS + q` |
| [103-size-and-window-queries](103-size-and-window-queries.sh) | Window operations `CSI 11/13/14/16/18/19 t`, decoded-as-escaped replies |
| [110-window-title](110-window-title.sh) | OSC 0/1/2; BEL and ST terminators; title push/pop `CSI 22/23;0 t` |
| [111-window-geometry](111-window-geometry.sh) | Move `CSI 3;x;y t`; pixel resize `4;h;w t`; character resize `8;rows;cols t` |
| [112-window-state](112-window-state.sh) | Deiconify/iconify 1/2; raise/lower 5/6; refresh 7; maximize 9; fullscreen 10 (`CSI … t`) |
| [120-hyperlinks](120-hyperlinks.sh) | OSC 8 open/close links, optional `id` parameter |
| [121-shell-integration](121-shell-integration.sh) | OSC 7 working directory; OSC 133 A/B/C/D command boundaries/status |
| [122-vendor-shell-integration](122-vendor-shell-integration.sh) | Representative OSC 633 A/B/C/D; OSC 1337 CurrentDir |
| [130-palette-colors](130-palette-colors.sh) | OSC 4 index set/query; OSC 104 index reset |
| [131-default-colors](131-default-colors.sh) | OSC 10/11 set/query; OSC 110/111 reset |
| [132-selection-colors](132-selection-colors.sh) | OSC 17/19 set/query; OSC 117/119 reset, followed by restoration |
| [140-clipboard](140-clipboard.sh) | OSC 52 set/query clipboard; primary-selection selector explained |
| [141-notifications](141-notifications.sh) | OSC 9, OSC 777 notify, OSC 99 |
| [142-progress-indicator](142-progress-indicator.sh) | OSC 9;4 states 0–4 |

### Input, character sets, rectangles, graphics, resets

| Demo | Sequences / features |
| --- | --- |
| [150-keyboard-modes](150-keyboard-modes.sh) | DECCKM `?1h/l`; keypad `ESC =/>`; arrow/navigation/function-key input |
| [151-modify-other-keys](151-modify-other-keys.sh) | xterm `CSI >4;n m` |
| [152-kitty-keyboard](152-kitty-keyboard.sh) | Query `CSI ? u`, push `>flags u`, set `=flags;mode u`, pop `<n u` |
| [153-bracketed-paste](153-bracketed-paste.sh) | `?2004h/l`; incoming `CSI 200~/201~` |
| [154-focus-events](154-focus-events.sh) | `?1004h/l`; incoming `CSI I/O` |
| [155-mouse-tracking](155-mouse-tracking.sh) | Modes 9, 1000, 1002, 1003; SGR button/motion/release reports |
| [156-mouse-encodings](156-mouse-encodings.sh) | Legacy encoding; modes 1005, 1006, 1015, 1016 |
| [160-character-sets](160-character-sets.sh) | G0/G1 `ESC ( B/0`, `ESC ) B/0`; SI/SO; G2/G3 designation and SS2/SS3 `ESC N/O` |
| [161-encoding-mode](161-encoding-mode.sh) | UTF-8/legacy selection `ESC % G/@` |
| [170-rectangular-erase-fill](170-rectangular-erase-fill.sh) | DECFRA `$x`, DECERA `$z`, DECSERA `${` |
| [171-rectangular-copy-attributes](171-rectangular-copy-attributes.sh) | DECCRA `$v`, DECCARA `$r`, DECRARA `$t` |
| [172-line-dimensions](172-line-dimensions.sh) | `ESC # 3/4/5/6` |
| [180-sixel](180-sixel.sh) | `DCS … q` raster sample |
| [181-regis](181-regis.sh) | `DCS 0 p` vector sample |
| [182-kitty-graphics](182-kitty-graphics.sh) | `APC G` PNG transmission, placement, deletion |
| [183-iterm-inline-image](183-iterm-inline-image.sh) | `OSC 1337;File=…:base64` |
| [190-control-strings](190-control-strings.sh) | SOS `ESC X`, PM `ESC ^`, APC `ESC _`, ST terminators |
| [191-printer-control](191-printer-control.sh) | Media copy / printer controller `CSI 0/4/5 i` |
| [900-palette-reset](900-palette-reset.sh) | OSC 104 whole-palette reset |
| [901-column-mode](901-column-mode.sh) | DECCOLM `?3h/l` |
| [902-screen-alignment](902-screen-alignment.sh) | DECALN `ESC # 8` |
| [903-scrollback-erasure](903-scrollback-erasure.sh) | ED extension `CSI 3 J` |
| [904-soft-reset](904-soft-reset.sh) | DECSTR `CSI ! p` |
| [905-full-reset](905-full-reset.sh) | RIS `ESC c` |

## Maintenance

`_lib.sh` contains shared I/O, bounded query/event reading, navigation, and cleanup;
`_image.sh` contains the tiny embedded checkerboard PNG. The runner discovers
numbered scripts automatically and runs each in a separate Bash process. Reserved
child exit codes 10/20/30 mean repeat/quit/previous.
The runner owns a temporary file for the shared delay setting and removes it on
exit. Arrow-key parsing accepts both normal and application cursor-key sequences.

For automated pseudo-terminal checks, `ANSI_DEMO_NO_PAUSE=1` skips the final key
wait, and `ANSI_DEMO_EVENT_SECONDS=0` skips timed input windows. These do not turn
the scripts into plain-text output: stdin and stdout must still be terminals.
