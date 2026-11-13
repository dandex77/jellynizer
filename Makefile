# Jellynizer Makefile

.PHONY: help run frontend-dev build test docker-build docker-run docker-up docker-down clean

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; ${NF}==1 {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}    '

run: ## Run the Rust backend server
	cargo run

frontend-dev: ## Run the React frontend development server
	cd frontend && npm run dev

build: ## Build the Rust backend in release mode
	cargo build --release

test: ## Run all backend tests
	cargo test

docker-build: ## Build the Docker image
	docker build -t jellynizer:latest . --no-cache

docker-up: ## Run the project with Docker Compose
	docker-compose up -d

docker-down: ## Stop and remove the Docker Compose containers
	docker-compose down

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf frontend/dist/