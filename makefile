build-ios:
	cargo build --target aarch64-apple-ios

publish:
	cargo publish
	npm publish

publish-dry-run:
	cargo publish --dry-run
	npm publish --dry-run