default:
  just --list

all: build test clippy fmt-check

build:
  cargo build

check:
 cargo check

clippy:
  cargo clippy --all-targets --all-features

fmt:
  cargo +nightly fmt

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

forbid:
  ./bin/forbid

run *args:
  cargo run -- {{args}}

test:
  cargo test

usage:
  cargo run -- --help | pbcopy

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
