set shell := ["cmd.exe", "/c"]
alias b := build
alias t := test
alias d := dev
alias r := run

build:
  @cargo build
test:
  @cargo test
dev:
  @cargo run
run:
  @cargo run --release
