DOCSDIR  = docs

TEST_CRATES = jacana jacana-api jacana-bot jacana-plugin
TEST_CMD  = cargo test -p

# Generic test target: make test-<package>
test-%:
	$(TEST_CMD) $* --lib -- --show-output

# Generic script command for package/jacana: make web-lint would lint the packages/jacana
pkg-%:
	(cd $(DOCSDIR) && bun run $*)

pkg:
	(cd $(DOCSDIR) && $(CMD))
