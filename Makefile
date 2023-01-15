up:
	docker-compose up -d
down:
	docker-compose down
build:
	docker-compose build
logs-web:
	docker-compose logs -f app
logs-db:
	docker-compose logs -f db
bash:
	docker-compose exec app bash
makemigration:
	@read -p "Enter Migration Name: " name; \
	docker-compose exec app bash -c "diesel migration generate $$name";
migrate:
	docker-compose exec app bash -c "diesel migration run"
migrate-redo:
	docker-compose exec app bash -c "diesel migration redo"
test:
	docker-compose exec app bash -c "cargo test -- --nocapture"