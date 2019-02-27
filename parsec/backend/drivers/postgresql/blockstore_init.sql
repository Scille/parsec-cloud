-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


CREATE TABLE blockstore (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    block_id UUID UNIQUE NOT NULL,
    block BYTEA NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL
);


-- TODO: continue blockstore rework...

-- CREATE TABLE blocks (
--     _id SERIAL PRIMARY KEY,
--     organization INTEGER REFERENCES organization (_id) NOT NULL,
--     beacon INTEGER REFERENCES beacons (_id) NOT NULL,
--     block_id UUID UNIQUE NOT NULL,
--     block BYTEA NOT NULL,
--     author INTEGER REFERENCES device (_id) NOT NULL,
--     deleted BOOLEAN NOT NULL
-- );


-- -- Only used if we store blocks' data in database
-- CREATE TABLE blocks_data (
--     _id SERIAL PRIMARY KEY,
--     data BYTEA NOT NULL,
-- );
