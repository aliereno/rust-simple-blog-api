up:
	docker-compose -f .docker/docker-compose.yml up -d
down:
	docker-compose -f .docker/docker-compose.yml down
build:
	docker-compose -f .docker/docker-compose.yml build
logs-web:
	docker-compose -f .docker/docker-compose.yml logs -f app
logs-db:
	docker-compose -f .docker/docker-compose.yml logs -f db
docker-bash:
	docker-compose -f .docker/docker-compose.yml exec app bash