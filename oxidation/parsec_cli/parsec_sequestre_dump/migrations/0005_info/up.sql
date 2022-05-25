-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS info (
    magic INTEGER UNIQUE NOT NULL DEFAULT 87947 PRIMARY KEY,
    version INTEGER NOT NULL,  -- should be 1 for now
    realm_id BLOB NOT NULL
);

INSERT INTO info VALUES (
    87947, 1, 'realm_id'
);


INSERT INTO realm_role VALUES (
    0, 'role'
);

INSERT INTO user_ VALUES (
    0, 'user'
);

INSERT INTO device VALUES (
    0, 'device'
);

INSERT INTO block VALUES (
    0, 'block_id', 'data', 0, 4, 0
);

INSERT INTO vlob_atom VALUES (
    0, 'vlob_id', 1, 'blob', 4, 0, 0
);
