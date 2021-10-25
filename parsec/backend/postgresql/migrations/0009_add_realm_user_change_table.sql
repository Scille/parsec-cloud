-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-------------------------------------------------------
--  Migration
-------------------------------------------------------

CREATE TABLE realm_user_change (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- The last time this user changed the role of another user
    last_role_change TIMESTAMPTZ,
    -- The last time this user updated a vlob
    last_vlob_update TIMESTAMPTZ,

    UNIQUE(realm, user_)
);
