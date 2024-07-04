# About Parsec data model

## Data model

The `datamodel.sql` script provides a full, up-to-date, overview of Parsec
data model. It is used mainly as a reference and to test migrations scripts,
but it is NOT used to initialize the database.

## Migrations

The SQL scripts named `xxxx_<name>.sql` are migrations scripts that are applied
in ascending order to initialize the database.

So to get a brand new database:

1. Apply `0001_initial.sql`
2. Apply `0002_add_field_X.sql`
3. and so on...

## Tests

Migrations are tested by applying them to a brand new database, dumping the
resulting database schemas and comparing them with `datamodel.sql` to
ensure consistency (see `./server/tests/test_migrations.py`).
