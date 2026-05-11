VERSION = $(shell cargo pkgid | sed 's!.*\@!!')

.PHONY: version check prerelease release

version:
	@echo "$(VERSION)"

run:
	GOLDFISH_LOG_LEVEL=info cargo run

prerelease:
	cargo generate-lockfile
	cargo build --release --locked
	cargo test
	cargo check
	cargo fmt
	cargo clippy -- -D warnings

check: prerelease

release: prerelease
	git tag v$(VERSION);
	git push origin --tags
