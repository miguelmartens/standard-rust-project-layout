# A front door for `make` muscle memory. It contains no automation.
#
# Every recipe in this file is a single line that forwards to `cargo xtask` or
# to cargo itself. That is the entire design, and this is the rule that keeps it
# honest:
#
#     If a recipe needs a second line, it has become automation,
#     and automation belongs in xtask/ -- cross-platform and type checked.
#
# See "Automation: `xtask`, not `make`" in the README for the argument. Nothing
# here requires make: `cargo xtask ci` is the supported entry point, and it is
# what Windows contributors and CI use. This file is a convenience, and deleting
# it costs nothing.
#
# GNU make. macOS ships 3.81, which is ancient and still has everything used
# below.

.DEFAULT_GOAL := help

# Every target here is a verb, not a file. Without .PHONY a directory named
# `test` in the repository root would silently stop `make test` from running --
# make would decide the target was already up to date and do nothing. This is
# the single most common Makefile bug and it fails quietly.
.PHONY: help ci fmt lint test build doc clean

MAKEFLAGS += --no-print-directory

help: ## Show this help
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / \
		{printf "  \033[36m%-6s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

ci: ## Everything CI runs: fmt, prettier, clippy, tests, doctests, rustdoc
	cargo xtask ci

fmt: ## Format in place -- rustfmt, then prettier if it is installed
	cargo xtask fmt

lint: ## Clippy over every target, warnings denied
	cargo xtask lint

test: ## Tests, then doctests
	cargo xtask test

build: ## Debug build of the whole workspace
	cargo build --workspace

doc: ## Build the API documentation and open it
	cargo doc --workspace --no-deps --open

clean: ## Remove target/
	cargo clean
