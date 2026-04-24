
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

check:
	cargo check --features server
	cargo check --features web
	cargo check --features desktop
	cargo check --features mobile

clippy:
	cargo clippy --features server
	cargo clippy --features web
	cargo clippy --features desktop
	cargo clippy --features mobile

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

fmt:
	cargo fmt

doc:
	cargo doc

apply-patch:
	cargo patch-crate

