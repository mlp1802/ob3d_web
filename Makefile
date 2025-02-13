run-dev:
	docker compose up --build
run-prod:
	docker compose -f docker-compose-prod.yml up --build
