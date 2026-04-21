run-dev:
	docker compose up 
run-prod-daemon:
	docker compose -f docker-compose-prod.yml up -d
run-prod:
	docker compose -f docker-compose-prod.yml up

check:
	bash ./check.sh
build-image-no-cache:
	docker compose build --no-cache
build-image:
	docker compose build --no-cache

