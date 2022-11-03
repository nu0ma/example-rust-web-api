docker build . --no-cache -t db
docker run --rm --name db -e POSTGRES_PASSWORD=password db