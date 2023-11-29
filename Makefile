update:
	cargo run --bin aoc-update -- $(YEAR) $(DAY)
	cargo fmt --package aoc$(YEAR)

update-desc:
	cargo run --bin aoc-update -- $(YEAR) $(DAY) only-desc
	cargo fmt --package aoc$(YEAR)

update-input:
	cargo run --bin aoc-update -- $(YEAR) $(DAY) only-input
	cargo fmt --package aoc$(YEAR)

update-main:
	cargo run --bin aoc-update -- $(YEAR) only-main
	cargo fmt --package aoc$(YEAR)

run:
	cargo run --bin aoc$(YEAR)

run-rls:
	cargo run --release --bin aoc$(YEAR)

bench:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin aoc$(YEAR) -- bench

bench-md:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --bin aoc$(YEAR) -- bench-md > crates/aoc$(YEAR)/README.md

build:
	cargo build --release

build-native:
	RUSTFLAGS="-C target-cpu=native" cargo build --release
