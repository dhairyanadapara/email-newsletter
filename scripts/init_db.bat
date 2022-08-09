
SET DB_USER=postgres
SET DB_PASSWORD=password
SET DB_NAME=newsletter
SET DB_PORT=5432

docker "run" "-e" "POSTGRES_USER=%DB_USER%" "-e" "POSTGRES_PASSWORD=%DB_PASSWORD%" "-e" "DB_NAME=%DB_NAME%" "-p" "%DB_PORT%":5432 "-d" "postgres" "postgres" "-N" "1000"

SET PGPASSWORD="%DB_PASSWORD%""


echo "Postgres is up and running on port %DB_PORT% - running migrations now!"
SET "DATABASE_URL=postgres://%DB_USER%:%DB_PASSWORD%@localhost:%DB_PORT%/%DB_NAME%"
sqlx database create
sqlx migrate run
echo "Postgres has been migrated, ready to go!"