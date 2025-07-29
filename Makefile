CRATES_DIR = $(CURDIR)/crates
TMP_DATABASE = $(CURDIR)/tmp.db

# Define default target
.PHONY: default
default: release

.PHONY: format-rust lint-all lint test release release-cuda
format-rust:
	cargo fmt

lint-all:
	pre-commit run --all-files

lint:
	pre-commit run

test:
	cargo test

release:
	cargo build --locked --profile=optimized-release --package=sauropod-inference-server

release-cuda:
	cargo build --locked --profile=optimized-release --features=cuda --package=sauropod-inference-server

# sqlx-related targets
.PHONY: sqlx-migrate sqlx-set-up sqlx-reset sqlx-prepare show-current-schema

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
	sqlite3 $(TMP_DATABASE) ".schema"
