test:
	cargo test -- --nocapture

watch-test:
	fd -e rs | entr -r make test
