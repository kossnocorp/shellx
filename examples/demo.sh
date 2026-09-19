#!/usr/bin/env bash
set -euo pipefail

# Run from the repository root, regardless of the current working directory.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

unset NO_COLOR

# All foreground colors
cargo run --bin shx '<black>black</black> <red>red</red> <green>green</green> <yellow>yellow</yellow> <blue>blue</blue> <magenta>magenta</magenta> <cyan>cyan</cyan> <white>white</white>'

# Bold and bright (aliases)
cargo run --bin shx 'Normal — <bold>bold</bold> — <bright>bright</bright>'

# Dim
cargo run --bin shx 'Normal — <dim>quiet, secondary text</dim>'

# Italic
cargo run --bin shx 'Hello, <italic>world!</italic>'

# Underline
cargo run --bin shx 'Read the <underline>important part</underline>.'

# Blink (terminal-dependent)
cargo run --bin shx '<blink>Attention!</blink>'

# Reverse foreground and background
cargo run --bin shx '<reverse> SELECTED </reverse> unselected'

# Hidden text (concealed visually, still present in output)
cargo run --bin shx 'Before [<hidden>secret text</hidden>] after'

# Strikethrough
cargo run --bin shx '<strikethrough>old plan</strikethrough> new plan'

# Nested styles
cargo run --bin shx '<red>Hello <bold>bold red <italic>and italic</italic></bold>, back to red</red>'

# Nested colors restore the parent color
cargo run --bin shx '<cyan>Cyan <magenta>magenta <yellow>yellow</yellow> magenta</magenta> cyan</cyan>'

# Reset temporarily clears all formatting, then restores the parent style
cargo run --bin shx '<bright><red>Bold red — <reset>plain text</reset> — bold red again</red></bright>'

# Combine color, bold, underline, and italic
cargo run --bin shx '<green><bold><underline>SUCCESS</underline></bold>: <italic>everything passed</italic></green>'

# Status-line example
cargo run --bin shx '<green><bold>PASS</bold></green> parser  <yellow><bold>WARN</bold></yellow> cache miss  <red><bold>FAIL</bold></red> connection refused'

# Multiple lines
cargo run --bin shx '<bold>Build report</bold>
<green>✓ Compilation succeeded</green>
<cyan>ℹ 5 tests passed</cyan>
<dim>Finished in 0.24s</dim>'

# Same markup with formatting disabled
NO_COLOR=1 cargo run --bin shx '<green><bold>SUCCESS</bold></green>: <italic>plain output</italic>'

# Some terminals ignore blink, italic, or hidden; black may be difficult to see
# on a dark background.
