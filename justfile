all: build test clippy fmt-check

build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

run subcommand *args:
	cargo run {{subcommand}} {{args}}

fmt:
	cargo +nightly fmt

check:
 cargo check

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"
