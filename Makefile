
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

clean:
	@cargo clean
	@rm -f z.*
	@rm -f *.profraw

clippy:
	cargo clippy --offline --tests --workspace

fmt:
	cargo fmt

doc:
	cargo doc

