.PHONY: signaling consumer test fmt clippy check

p:
	ffmpeg -hide_banner -loglevel error -f v4l2 -i /dev/video0 -c:v libx264 -preset veryfast -tune zerolatency -fflags nobuffer -flags low_delay -f mpegts - | cargo run --bin producer -- p

s:
	cargo run --bin consumer -- s

c:
	cargo run --bin consumer -- c | ffplay -fflags nobuffer -flags low_delay -framedrop -i -

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check --all-targets --all-features

ci:
	make fmt clippy check test

netem:
	sudo tc qdisc add dev lo root netem delay 80ms 20ms
	# sudo tc qdisc add dev lo root netem delay 80ms 30ms loss 5% reorder 25% 50%

unetem:
	sudo tc qdisc del dev lo root
