# Define variables
SCHEMA_DIR = $(CURDIR)/api
PACKAGES_DIR = $(CURDIR)/packages
SCHEMA_FILE = $(SCHEMA_DIR)/openapi.json

# Define default target
.PHONY: default
default: release

# Internal targets
.PHONY: _generate-code-from-structs _generate-ts-clients _prettier
_generate-code-from-structs:
	cargo run --bin generate-code-from-structs -- "$(SCHEMA_FILE)"

_generate-ts-clients:
	npx openapi-typescript

_prettier:
	npx prettier --write $(PACKAGES_DIR)/client $(SCHEMA_DIR)

# Public targets
.PHONY: generate format-rust lint-all lint test release ui
generate: _generate-code-from-structs _generate-ts-clients _prettier format-rust

format-rust:
	cargo fmt

lint-all:
	pre-commit run --all-files

lint:
	pre-commit run

test:
	npm run test
	cargo test

release: generate ui
	cargo build --locked --profile=optimized-release --package sauropod-server

ui:
	npm run build
