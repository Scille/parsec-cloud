-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------


CREATE TABLE human (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    email VARCHAR(254) NOT NULL,
    label VARCHAR(254) NOT NULL,

    UNIQUE(organization, email)
);

ALTER TABLE user_ ADD human INTEGER REFERENCES human (_id);
