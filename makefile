SHELL = /bin/sh

help: ## This help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run: ## Run docker container
	docker compose up --build -d

run-force: ## Run docker container
	docker compose up --build -d --force-recreate

stop: ## Stop docker container
	docker compose stop

down: ## Drop docker container
	docker compose down

cli: ## Run shell inside docker container
	docker compose exec app bash

build: ## Build project
	docker compose exec app cargo run

run-dev: ## Run docker container develop
	docker compose -f docker-compose-dev.yml up --build -d

run-dev-force: ## Run docker container develop
	docker compose -f docker-compose-dev.yml up --build -d --force-recreate

stop-dev: ## Stop docker container develop
	docker compose -f docker-compose-dev.yml stop

down-dev: ## Drop docker container develop
	docker compose -f docker-compose-dev.yml down

cli-dev: ## Run shell inside docker container develop
	docker compose -f docker-compose-dev.yml exec app bash

build-dev: ## Build project develop
	docker compose -f docker-compose-dev.yml exec app cargo run
