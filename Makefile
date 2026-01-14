.phony:: misc test watch-test


CUECMD = cue export --out text ./cue:documentation
CUEFILES = $(wildcard cue/*.cue)
generate: LICENSE README.md CHANGELOG.md generated/help.rs

test:
	cargo test -- --nocapture

test-sort:
	cargo test -- --test-threads=1 --nocapture

watch-test:
	fd -e rs | entr -r make test

README.md: $(CUEFILES)
	$(CUECMD) -e readme.full > $@

CHANGELOG.md: $(CUEFILES)
	$(CUECMD) -e changelog.text > $@

generated/help.rs: $(CUEFILES)
	-mkdir -p $(dir $@)
	$(CUECMD) -e help.code > $@

LICENSE: cue/LICENSE.cue
	cue export $< --out text -e license > $@

