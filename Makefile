makemigration:
	@read -p "Enter Migration Name: " name; \
	diesel migration generate $$name"
migrate:
	diesel migration run
migrate-redo:
	diesel migration redo
test:
	cargo test -- --nocapture
lint:
	cargo fmt --all --
	cargo clippy --version
	cargo clippy --tests -- -D warnings