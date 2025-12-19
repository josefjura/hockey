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

precommit: format-check clippy test ## Run all pre-commit checks (format, clippy, test)
	@echo "$(GREEN)✓ All pre-commit checks passed!$(NC)"

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

test: ## Run tests
	@echo "$(YELLOW)==> Running tests...$(NC)"
	@cargo test

##@ Build Commands

build: ## Build for production
	@echo "$(YELLOW)==> Building for production...$(NC)"
	@cargo build --release

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

