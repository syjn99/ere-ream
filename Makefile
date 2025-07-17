#!/usr/bin/make -f

# Makefile from [SuccinctPaul/ere-zkvm-examples](https://github.com/SuccinctPaul/ere-zkvm-examples) with a few modifications

.DEFAULT_GOAL := help

##@ Help
.PHONY: help
help: # Display this help.
	@awk 'BEGIN {FS = ":.*#"; printf "Usage:\n  make \033[34m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?#/ { printf "  \033[34m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }' $(MAKEFILE_LIST)

##@ build
.PHONY: build lint
build: ## cargo build
	@echo "Building project..."
	cargo build --release

##@ runner
.PHONY: run_sp1 run_risc0 run_openvm run_pico
run_sp1: ## run sp1
	cargo run --bin host --features=sp1  --release -- --nocapture

run_risc0: ## run risc0
	cargo run --bin host --features=risc0  --release -- --nocapture

run_openvm: ## run openvm
	cargo run --bin host --features=openvm  --release -- --nocapture

# Note: Pico requires specific nightly version.
run_pico: ## run pico
	cargo +nightly-2024-11-27 run --bin host --features=pico  --release -- --nocapture

run_zisk: ## run zisk
	cargo run --bin host --features=zisk  --release -- --nocapture