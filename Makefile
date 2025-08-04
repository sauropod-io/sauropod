CRATES_DIR = $(CURDIR)/crates
TMP_DATABASE = $(CURDIR)/tmp.db

# Define default target
.PHONY: default
default: release

# Codebase targets
.PHONY: format-rust lint fix test generate-dockerfiles
format-rust:
	cargo fmt

lint:
	pre-commit run --all-files

fix: lint
	cargo clippy --fix --allow-dirty --allow-staged --workspace -- --deny=clippy::all

test:
	cargo test

generate-dockerfiles:
	cargo run -p generate-dockerfiles

# Artifact building targets
.PHONY: release release-cuda docker-vulkan docker-cuda
release:
	@rust_sysroot=$$(rustc --print sysroot); \
	rust_lld_wrapper=$$(find $$rust_sysroot -name ld.lld | head -n 1); \
	export SOURCE_DATE_EPOCH="$$(git log -1 --pretty=%ct)"; \
	if [ -z "$$rust_lld_wrapper" ]; then \
	  echo "Error: Could not find ld.lld in $$rust_sysroot"; \
	  exit 1; \
	fi; \
	if [ -z "$$SOURCE_DATE_EPOCH" ]; then \
	  export SOURCE_DATE_EPOCH=0; \
	fi; \
	export \
		CARGO_INCREMENTAL=0 \
		CC=$${CC:-clang} \
		CFLAGS="-ffile-prefix-map=$(CURDIR)=. -fdebug-prefix-map=$(CURDIR)=. -g0"; \
		CXX=$${CXX:-clang++} \
		CXXFLAGS="$${CFLAGS}" \
		RUSTFLAGS="$$RUSTFLAGS --remap-path-prefix=$(CURDIR)=. -Clinker-plugin-lto -Clink-arg=-Wl,-rpath,\$$ORIGIN" \
		TZ=UTC; \
	if [ "$$(uname)" = "Linux" ]; then \
		export \
			LDFLAGS="-fuse-ld=lld"; \
			RUSTFLAGS="$$RUSTFLAGS -Clink-arg=-fuse-ld=lld -Clink-arg=-Wl,--threads=4" \
			PATH=$$(dirname $$rust_lld_wrapper):$${PATH}; \
	fi; \
	export CXXFLAGS="$${CFLAGS}"; \
	cargo build --locked --profile=optimized-release \
	  --no-default-features \
	  --package=sauropod-inference-server \
	  --features="$${CARGO_FEATURES}"

release-vulkan:
	$(MAKE) release CARGO_FEATURES=vulkan

release-cuda:
	$(MAKE) release CARGO_FEATURES=cuda-multiple-arches

docker-vulkan:
	docker build -t ghcr.io/sauropod-io/sauropod:latest -f docker/Dockerfile.vulkan .

docker-cuda:
	docker build -t ghcr.io/sauropod-io/sauropod:latest -f docker/Dockerfile.cuda .

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
