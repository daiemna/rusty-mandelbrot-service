.DEFAULT_GOAL := help
PROJECT_NAME:=$(shell grep -E -i -m 1 "^name( )?=( )?" Cargo.toml | sed -e "s/\"//g" | sed -e "s/name.*=\ //g")
ALL_RUST_FILES=$(shell find ./src -type f -name "*.rs" 2> /dev/null)

help: ## Show available options with this Makefile
	@grep -F -h "##" $(MAKEFILE_LIST) | grep -v grep | awk 'BEGIN { FS = ":.*?##" }; { printf "%-18s  %s\n", $$1,$$2 }'

.PHONY: test
test: ## runs all the tests using pytest.
	cargo test

.PHONY: run
run: ## runs the application
	cargo run

.PHONY: reset_lock_file
reset_lock_file: ## resets lock file, use it in case lock file hashes have issues.
	rm -f Cargo.lock

.PHONY: lint
lint: ## Run all the lint tasks. Please check setup.cfg for any configuration changes
	cargo clippy

.PHONY: format
format: ## Run isort, autoflake and black to fix linting/formatting errors automatically
	rustfmt --edition 2021 ${ALL_RUST_FILES}

.PHONY: release
release: ## creates distribution files
	cargo build --release

.PHONY: clean
clean: ## cleans extra files
	cargo clean

.PHONY: dockerized
dockerized: ## builds docker image
	docker-compose down --remove-orphans && \
	DOCKER_BUILDKIT=0 docker-compose up --build -d

.PHONY: install_git_hooks
install_git_hooks:
	pre-commit install && \
	pre-commit install --hook-type pre-push --hook-type prepare-commit-msg

.PHONY: precommit
precommit: ## run precommit on all files.
	pre-commit run --all-files

# .PHONY: dev_setup
# dev_setup: pre_install ## install all the dependencies inside python environment, run this after creating and activating you environment
# 	poetry install -vvv && \
# 	make install_git_hooks

# .PHONY: test_setup
# test_setup: pre_install ## development setup and docker if need
# 	poetry install -vv --without=dev
