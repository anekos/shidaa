
SRCS := $(wildcard $(src/*.rs))

release: $(SRCS)
	cargo build --release

sample.png: $(SRCS)
	cargo run -- --width 400 --height 400 sample.png
