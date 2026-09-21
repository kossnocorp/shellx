#!/usr/bin/env bash
set -euo pipefail

# Run from the repository root, regardless of the current working directory.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

unset NO_COLOR

# All foreground colors
cargo run --bin shx '<black>black</black> <red>red</red> <green>green</green> <yellow>yellow</yellow> <blue>blue</blue> <magenta>magenta</magenta> <cyan>cyan</cyan> <white>white</white>'

# Bold and bright colors are independent
cargo run --bin shx '<red>red</red> — <bold><red>bold red</red></bold> — <bright><red>bright red</red></bright>'

# Gray aliases bright black (ANSI 90)
cargo run --bin shx '<gray>gray</gray> — <bright><black>bright black</black></bright> — <bold><gray>bold gray</gray></bold>'

# All bright foreground colors
cargo run --bin shx '<bright><black>black</black> <red>red</red> <green>green</green> <yellow>yellow</yellow> <blue>blue</blue> <magenta>magenta</magenta> <cyan>cyan</cyan> <white>white</white></bright>'

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
cargo run --bin shx '<bright><red>Bright red — <reset>plain text</reset> — bright red again</red></bright>'

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

# Span attributes and background colors; bright applies to both colors
cargo run --bin shx '<span fg=white bg=blue bold bright> Highlighted </span> <span fg=gray italic>secondary text</span>'

# Color tags accept attributes too
cargo run --bin shx '<red bold dim bg=green>Hello</red>'

# Explicit false temporarily disables inherited styles
cargo run --bin shx '<span fg=cyan bold italic>Styled <span bold=false italic=false>plain weight</span> styled again</span>'

# Unknown attributes are ignored; quoted values are supported
cargo run --bin shx '<span fg="green" underline=true ignored="anything">Underlined green</span>'

# Some terminals ignore blink, italic, or hidden; black may be difficult to see
# on a dark background.

# Sequential, indexed, and named placeholders
cargo run --bin shx 'Hello, {}! {}' 'Sasha' 'How are you?'
cargo run --bin shx 'Hello, {1}! {0}' 'How are you?' 'Sasha'
cargo run --bin shx '{hello}, {name}! {question}' name='Sasha' hello='Hi' question='How are you?'

# Values inherit formatting; double braces produce literal braces
cargo run --bin shx '<green bold>Hello, {name}!</green> {{welcome}}' name='Sasha'
