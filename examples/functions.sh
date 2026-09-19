#!/usr/bin/env bash

set -euo pipefail

eval "$(cargo run --quiet --bin shx -- fn say_hello \
  '<red>Hello, <green>{0}</green></red>')"

say_hello "Sasha"
say_hello "Friend"

# Automatic placeholders enumerate arguments: {} {} is equivalent to {0} {1}.
eval "$(cargo run --quiet --bin shx -- fn greet_pair \
  '<red>Hello, <green>{}</green> and <green>{}</green></red>')"

greet_pair "Sasha" "Friend"
