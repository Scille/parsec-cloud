-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

-- This migration simply creates a new table to keep track of the
-- timestamp of the last changes for a given user on a given realm.
-- Note that this table is not populated during the migration.
-- This is because those values are only used to deal with concurrency
-- issues and hence are only useful during the current ballpark.
-- Waiting a full ballpark during next migration should then be enough
-- to avoid any migration issue in this regard. If those values end up
-- being used for something else in the future, another migration script
-- will have to be written.


CREATE TABLE realm_user_change (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- The last time this user changed the role of another user
    -- The value might be NULL, if `last_vlob_update` has been set first
    last_role_change TIMESTAMPTZ,
    -- The last time this user updated a vlob
    -- The value might be NULL, if `last_role_change` has been set first
    last_vlob_update TIMESTAMPTZ,

    UNIQUE(realm, user_)
);
