watch:
	cargo watch -x test
doc:
	cargo doc --no-deps  --all-features  --document-private-items  --open
bench:
	cargo bench --all-features
test:
	cargo test --all-features
