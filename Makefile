# Define variables
SCHEMA_DIR = $(CURDIR)/api
DOCS_DIR = $(CURDIR)/docs
PACKAGES_DIR = $(CURDIR)/packages
CRATES_DIR = $(CURDIR)/crates
SCHEMA_FILE = $(SCHEMA_DIR)/openapi.json
TMP_DATABASE = $(CURDIR)/tmp.db

# Define default target
.PHONY: default
default: release

# Internal targets
.PHONY: _generate-code _generate-ts-clients _prettier
_generate-code:
	cargo run --bin generate-code -- "$(SCHEMA_FILE)"

_generate-ts-clients:
	npx openapi-typescript

_prettier:
	npx prettier --write $(PACKAGES_DIR)/client $(SCHEMA_DIR) $(DOCS_DIR)

# Public targets
.PHONY: generate format-rust lint-all lint test release ui
generate: _generate-code _generate-ts-clients _prettier format-rust

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

# sqlx-related targets
.PHONY: sqlx-migrate sqlx-set-up

MIGRATIONS_DIR = $(CURDIR)/crates/database/migrations
sqlx-migrate:
	DATABASE_URL=sqlite://$(TMP_DATABASE) cargo --quiet run -p sqlx-wrapper -- migrate run --source "$(MIGRATIONS_DIR)"  --database-url "sqlite://$(TMP_DATABASE)"

sqlx-set-up:
	cargo --quiet run -p sqlx-wrapper -- database setup --source "$(MIGRATIONS_DIR)" --database-url "sqlite://$(TMP_DATABASE)"
	@echo "Now run export DATABASE_URL=sqlite://$(TMP_DATABASE)"

sqlx-reset:
	DATABASE_URL=sqlite://$(TMP_DATABASE) cargo --quiet run -p sqlx-wrapper -- database reset --source "$(MIGRATIONS_DIR)" -y

sqlx-prepare:
	DATABASE_URL=sqlite://$(TMP_DATABASE) cargo --quiet run -p sqlx-wrapper -- prepare --workspace

show-current-schema:
	sqlite3 ./tmp.db ".schema"
