run-dev:
	docker compose up 
run-prod:
	docker compose -f docker-compose-prod.yml up -d

check:
	bash ./check.sh
