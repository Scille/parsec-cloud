-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------

ALTER TABLE user_ ADD current_profile USER_PROFILE;

UPDATE user_
SET current_profile = COALESCE(
    (
        SELECT profile.profile
        FROM profile
        WHERE profile.user_ = user_._id
        ORDER BY profile.certified_on DESC
        LIMIT 1
    ),
    initial_profile
);

ALTER TABLE user_ ALTER COLUMN current_profile SET NOT NULL;
