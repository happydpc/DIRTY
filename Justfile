# wengwengweng

run: build
	cargo run --release --example window

run-lua: build
	./bin/dirty examples/main.lua

build:
	cargo build --release
	rm -rf bin/dirty
	cp target/release/dirty bin/dirty
	strip bin/dirty

doc:
	cargo doc --no-deps --open

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

