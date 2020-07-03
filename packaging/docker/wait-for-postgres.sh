#!/bin/sh
# wait-for-postgres.sh

# Make Parsec wait for Postgres as it is not resilient enough
# As recommended as a quickfix even by Docker documentation
# Will be rendered useless when docker-compose is updated
# See https://docs.docker.com/compose/startup-order/

set -e

host="$1"
shift
cmd="$@"

apt-get update -y; apt-get install postgresql-client -y  # I'm ashamed.

until psql "$host" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"
exec $cmd
