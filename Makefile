.PHONY: check check-headless check-desktop dev format test

check: check-headless check-desktop
	corepack pnpm check
	corepack pnpm build

check-headless:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings
	cargo test

check-desktop:
	cargo clippy -p lume-desktop --all-targets -- -D warnings

dev:
	corepack pnpm tauri dev

format:
	cargo fmt --all
	corepack pnpm --filter @lume/desktop-ui exec prettier --write .

test:
	cargo test
