run-dev:
	docker compose up 
run-prod-daemon:
	docker compose -f docker-compose-prod.yml up -d
run-prod:
	docker compose -f docker-compose-prod.yml up

check:
	bash ./check.sh
