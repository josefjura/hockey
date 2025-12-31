.PHONY: help precommit check lint format format-check test build dev clean install docker-build docker-up docker-down create-admin

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

help: ## Show this help message
	@echo '$(GREEN)Available targets:$(NC)'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'

##@ Pre-commit Commands

precommit: ## Run all pre-commit checks and fixes (REQUIRED before push!)
	@echo "$(YELLOW)==> Running pre-commit checks...$(NC)"
	@echo "$(YELLOW)==> 1/4: Formatting Rust code...$(NC)"
	@cargo fmt
	@echo "$(YELLOW)==> 2/4: Running Clippy...$(NC)"
	@cargo clippy -- -D warnings
	@echo "$(YELLOW)==> 3/4: Running Rust tests...$(NC)"
	@cargo test --quiet
	@echo "$(YELLOW)==> 4/4: Checking for unstaged changes...$(NC)"
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "$(RED)⚠ Warning: You have unstaged changes (possibly from cargo fmt)$(NC)"; \
		echo "$(YELLOW)Please review and commit them before pushing.$(NC)"; \
	fi
	@echo "$(GREEN)✓ All pre-commit checks passed!$(NC)"
	@echo "$(GREEN)✓ Ready to commit and push!$(NC)"

##@ Development Commands

dev: ## Start development server
	@echo "$(YELLOW)==> Starting development server...$(NC)"
	@cargo run

create-admin: ## Create an admin user
	@echo "$(YELLOW)==> Creating admin user...$(NC)"
	@cargo run --bin create_admin

##@ Code Quality Commands

format: ## Format code with rustfmt
	@echo "$(YELLOW)==> Formatting code...$(NC)"
	@cargo fmt

format-check: ## Check code formatting without modifying
	@echo "$(YELLOW)==> Checking code formatting...$(NC)"
	@cargo fmt --check

clippy: ## Run Clippy linter
	@echo "$(YELLOW)==> Running Clippy...$(NC)"
	@cargo clippy -- -D warnings

lint: clippy ## Alias for clippy

check: ## Run cargo check
	@echo "$(YELLOW)==> Checking compilation...$(NC)"
	@cargo check

test: ## Run Rust tests
	@echo "$(YELLOW)==> Running Rust tests...$(NC)"
	@cargo test

test-storybook: ## Run Storybook component tests
	@echo "$(YELLOW)==> Running Storybook test-runner...$(NC)"
	@cd web_components && yarn test-storybook

test-e2e: ## Run E2E smoke tests (requires server running on :8080)
	@echo "$(YELLOW)==> Running E2E smoke tests...$(NC)"
	@yarn test:e2e

test-all: test test-storybook test-e2e ## Run all tests (Rust + Storybook + E2E)
	@echo "$(GREEN)✓ All tests passed!$(NC)"

##@ Build Commands

build: ## Build for production
	@echo "$(YELLOW)==> Building for production...$(NC)"
	@cargo build --release

build-full: ## Build for production with minified assets
	@echo "$(YELLOW)==> Building web components with minification...$(NC)"
	@cd web_components && yarn install && yarn build:prod
	@echo "$(YELLOW)==> Building Rust binary for production...$(NC)"
	@cargo build --release
	@echo "$(GREEN)✓ Production build complete!$(NC)"

##@ Docker Commands

docker-build: ## Build Docker image
	@echo "$(YELLOW)==> Building Docker image...$(NC)"
	@docker build -t hockey:latest .

docker-up: ## Start Docker containers
	@echo "$(YELLOW)==> Starting Docker containers...$(NC)"
	@docker compose up -d

docker-down: ## Stop Docker containers
	@echo "$(YELLOW)==> Stopping Docker containers...$(NC)"
	@docker compose down

docker-logs: ## Show Docker container logs
	@docker compose logs -f

##@ Utility Commands

clean: ## Clean build artifacts
	@echo "$(YELLOW)==> Cleaning build artifacts...$(NC)"
	@cargo clean

install: ## Install/update dependencies
	@echo "$(YELLOW)==> Updating dependencies...$(NC)"
	@cargo build

