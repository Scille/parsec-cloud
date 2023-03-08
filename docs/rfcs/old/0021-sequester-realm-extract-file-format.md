# Sequester - realm extract file format

From [ISSUE-2461](https://github.com/Scille/parsec-cloud/issues/2461)

## Filename

`parsec_sequester_realm_extract_<realm_id>.sqlite`

## Schemas

```sql
-- Metadata & Data export

CREATE TABLE block (
    -- _id is not SERIAL given we will take the one present in the Parsec database
    _id PRIMARY KEY,
    block_id UUID NOT NULL,
    data BYTEA NOT NULL,
    -- TODO: are author/size/created_on useful ?
    author INTEGER REFERENCES device (_id) NOT NULL,
    size INTEGER NOT NULL,
    -- this field is created by the backend when inserting (unlike vlob's timestamp, see below)
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(block_id)
);

CREATE TABLE vlob_atom (
    -- Compared to Parsec's datamodel, we don't store `vlob_encryption_revision` given
    -- the vlob is provided with third party encrytion key only once at creation time
    _id PRIMARY KEY,
    vlob_id UUID NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    size INTEGER NOT NULL,
    -- author/timestamp are required to validate the consistency of blob
    -- Care must be taken when exporting this field (and the device_ table) to
    -- keep this relationship valid !
    author INTEGER REFERENCES device (_id) NOT NULL,
    -- this field is called created_on in Parsec datamodel, but it correspond to the timestamp field in the API
    -- (the value is provided by the client when sending request and not created on backend side) so better
    -- give it the better understandable name
    timestamp TIMESTAMPTZ NOT NULL,

    UNIQUE(vlob_id, version)
);

-- user/device/realm_role certificates related to the current realm
--
-- There is no need for relationship between user/device given all those data
-- are small enough to have the script just load them once and kept them in memory
--
-- However we cannot just dump all the certificates in a single table given we cannot
-- preserve primary key when merging user/device/role tables together.

CREATE TABLE realm_role (
    _id PRIMARY KEY,
    role_certificate BYTEA NOT NULL
);

CREATE TABLE user_ (
    _id PRIMARY KEY,
    user_certificate BYTEA NOT NULL,
    revoked_user_certificate BYTEA  -- NULL if user not revoked
);

CREATE TABLE device (
    _id PRIMARY KEY,
    device_certificate BYTEA NOT NULL
);

-- Database info

-- This magic number has two roles:
-- - it makes unlikely we mistakenly consider an unrelated database as a legit
-- - it acts as a constant ID to easily retrieve the single row the table
CREATE TABLE IF NOT EXISTS info(
    magic INTEGER UNIQUE NOT NULL DEFAULT 87947,
    version INTEGER NOT NULL,  -- should be 1 for now
    realm_id UUID NOT NULL
);
```

## Format rational

Data to export are:

- realm vlobs
- realm blocks
- realm ID. This is needed to retrieve the root manifest of the realm.

So it is fairly simple to store those data as key/value, typically on the disk:

```txt
/<workspace_id>
/<workspace_id>/vlobs/
/<workspace_id>/vlobs/<vlob_id>
/<workspace_id>/vlobs/[...]
/<workspace_id>/blocks/
/<workspace_id>/blocks/<block_id>
/<workspace_id>/blocks/[...]
```

While simple, this brings multiples issues:

- a huge number of files per folder can lead to poor performances (or even errors !) depending on the OS
- copy of data is slower than of a single file. This is to consider given realm export contain all the history and hence can be fairly big.

For those reasons I think it would be better to export each realm as a separate SQLite database (data limit on sqlite is [pretty huge and well documented](https://www.sqlite.org/limits.html)).
