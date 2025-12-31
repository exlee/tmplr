.phony:: misc test watch-test

test:
	cargo test -- --nocapture

watch-test:
	fd -e rs | entr -r make test


misc: LICENSE

LICENSE: cue/LICENSE.cue
	cue export $< --out text -o LICENSE -e license

