VERSION = $(shell cargo pkgid | sed 's!.*#!!')

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
	git add Cargo.lock; \
	if git diff --cached --quiet; then \
		echo "Cargo.lock unchanged"; \
	else \
		git commit -m "Update Cargo.lock for v$(VERSION)"; \
	fi
	git tag v$(VERSION);
	git push origin --tags;
	git push origin
