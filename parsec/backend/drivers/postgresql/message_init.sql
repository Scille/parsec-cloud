-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


CREATE TABLE message (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    recipient INTEGER REFERENCES users (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    index INTEGER NOT NULL,
    sender INTEGER REFERENCES devices (_id) NOT NULL,
    body BYTEA NOT NULL
);
