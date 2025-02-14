run-dev:
	docker compose up 
run-prod:
	docker compose -f docker-compose-prod.yml up --build

check:
	bash ./check.sh
