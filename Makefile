.PHONY: build install uninstall build.release fmt check fix

build :
	@cargo build

install : build.release
	@cargo install --path .

uninstall :
	@cargo uninstall

build.release :
	@cargo build --release

fmt:
	@cargo fmt

check:
	@cargo clippy

fix:
	@cargo clippy --fix --allow-staged

test:
	@cargo test -- --test-threads=1