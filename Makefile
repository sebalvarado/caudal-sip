.DEFAULT_GOAL := help
GO ?= go

.PHONY: help
help: ## List available targets
	@grep -hE '^[a-z-]+:.*?## ' $(MAKEFILE_LIST) | awk -F':.*?## ' '{printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build all binaries into bin/
	$(GO) build -o bin/ ./cmd/...

.PHONY: test
test: ## Run tests with the race detector
	$(GO) test -race ./...

.PHONY: fmt
fmt: ## Format all Go source
	$(GO) fmt ./...

.PHONY: vet
vet: ## Run go vet
	$(GO) vet ./...

.PHONY: lint
lint: ## Run golangci-lint if it is installed
	@command -v golangci-lint >/dev/null 2>&1 \
		&& golangci-lint run \
		|| echo "golangci-lint not installed; skipping (CI runs it)"

.PHONY: tidy
tidy: ## Tidy and verify module dependencies
	$(GO) mod tidy
	$(GO) mod verify

.PHONY: check
check: vet test ## What CI runs: vet, then tests

.PHONY: clean
clean: ## Remove build artifacts
	rm -rf bin/
