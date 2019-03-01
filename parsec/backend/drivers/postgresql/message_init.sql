-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


CREATE TABLE message (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    recipient INTEGER REFERENCES user_ (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    index INTEGER NOT NULL,
    sender INTEGER REFERENCES device (_id) NOT NULL,
    body BYTEA NOT NULL
);
