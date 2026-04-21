curl -X POST http://localhost:8000/sync/check \
  -H "Content-Type: application/json" \
  -d '{"local_zip_file":"old.zip"}'
