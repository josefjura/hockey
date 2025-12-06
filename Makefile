.PHONY: help precommit precommit-web precommit-server check-web check-server lint-web lint-server format-web format-server test-web test-server build-web build-server

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

help: ## Show this help message
	@echo '$(GREEN)Available targets:$(NC)'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2}'

##@ Pre-commit Commands

precommit: precommit-web precommit-server ## Run all pre-commit checks (web + server)
	@echo "$(GREEN)✓ All pre-commit checks passed!$(NC)"

precommit-web: ## Run all pre-commit checks for frontend (lint, format check, type check, build)
	@echo "$(GREEN)==> Running frontend pre-commit checks...$(NC)"
	@$(MAKE) lint-web
	@$(MAKE) format-check-web
	@$(MAKE) typecheck-web
	@echo "$(GREEN)✓ Frontend pre-commit checks passed!$(NC)"

precommit-server: ## Run all pre-commit checks for backend (format, clippy, test)
	@echo "$(GREEN)==> Running backend pre-commit checks...$(NC)"
	@$(MAKE) format-check-server
	@$(MAKE) clippy-server
	@$(MAKE) test-server
	@echo "$(GREEN)✓ Backend pre-commit checks passed!$(NC)"

##@ Frontend Commands

lint-web: ## Run ESLint on frontend
	@echo "$(YELLOW)==> Linting frontend...$(NC)"
	@cd frontend && yarn lint

format-web: ## Format frontend code with Prettier
	@echo "$(YELLOW)==> Formatting frontend code...$(NC)"
	@cd frontend && yarn format

format-check-web: ## Check frontend code formatting without modifying
	@echo "$(YELLOW)==> Checking frontend code formatting...$(NC)"
	@cd frontend && yarn format:check

typecheck-web: ## Run TypeScript type checking
	@echo "$(YELLOW)==> Type checking frontend...$(NC)"
	@cd frontend && yarn typecheck

test-web: ## Run frontend tests
	@echo "$(YELLOW)==> Running frontend tests...$(NC)"
	@cd frontend && yarn test

build-web: ## Build frontend for production
	@echo "$(YELLOW)==> Building frontend...$(NC)"
	@cd frontend && yarn build

dev-web: ## Start frontend development server
	@cd frontend && yarn dev

##@ Backend Commands

format-server: ## Format backend code with rustfmt
	@echo "$(YELLOW)==> Formatting backend code...$(NC)"
	@cd backend && cargo fmt

format-check-server: ## Check backend code formatting without modifying
	@echo "$(YELLOW)==> Checking backend code formatting...$(NC)"
	@cd backend && cargo fmt --check

clippy-server: ## Run Clippy linter on backend
	@echo "$(YELLOW)==> Running Clippy on backend...$(NC)"
	@cd backend && cargo clippy -- -D warnings

test-server: ## Run backend tests
	@echo "$(YELLOW)==> Running backend tests...$(NC)"
	@cd backend && cargo test

build-server: ## Build backend for production
	@echo "$(YELLOW)==> Building backend...$(NC)"
	@cd backend && cargo build --release

check-server: ## Run cargo check on backend
	@echo "$(YELLOW)==> Checking backend compilation...$(NC)"
	@cd backend && cargo check

dev-server: ## Start backend development server
	@cd backend && cargo run

##@ Combined Commands

lint: lint-web clippy-server ## Run all linters (web + server)

format: format-web format-server ## Format all code (web + server)

format-check: format-check-web format-check-server ## Check all code formatting (web + server)

test: test-web test-server ## Run all tests (web + server)

build: build-web build-server ## Build all projects (web + server)

check: check-web check-server ## Run all checks (web + server)

check-web: ## Run all frontend checks (lint, format, typecheck)
	@$(MAKE) lint-web
	@$(MAKE) format-check-web
	@$(MAKE) typecheck-web

##@ Utility Commands

clean-web: ## Clean frontend build artifacts and dependencies
	@echo "$(YELLOW)==> Cleaning frontend...$(NC)"
	@cd frontend && rm -rf .next node_modules

clean-server: ## Clean backend build artifacts
	@echo "$(YELLOW)==> Cleaning backend...$(NC)"
	@cd backend && cargo clean

clean: clean-web clean-server ## Clean all build artifacts

install-web: ## Install frontend dependencies
	@echo "$(YELLOW)==> Installing frontend dependencies...$(NC)"
	@cd frontend && yarn install

install-server: ## Install/update backend dependencies
	@echo "$(YELLOW)==> Updating backend dependencies...$(NC)"
	@cd backend && cargo build

install: install-web install-server ## Install all dependencies
