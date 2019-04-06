
SRCS := $(wildcard $(src/*.rs)) Cargo.lock Cargo.toml

.PHONY: test

test: $(SRCS)
	cargo test

target/release/shidaa: $(SRCS)
	cargo build --release

sample.png: $(SRCS)
	cargo run -- --width 400 --height 400 sample.png
