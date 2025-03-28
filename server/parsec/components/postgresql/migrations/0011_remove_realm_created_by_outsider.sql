-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- In Parsec 3.3.0 the rule for creating a realm has been changed:
-- - Previously an Outsider was able to create a realm as long as he was not sharing
--   it with anyone.
-- - Now an Outsider is not allowed to create a realm.
--
-- The need to be able to create a realm as an Outsider was an artifact of
-- Parsec v2 and is not needed anymore (it was used to store the user manifest
-- which doesn't need to be synchronized at all in Parsec v3).
--
-- However in Parsec < 3.3.0, the client was still automatically creating a
-- realm for storing user manifest (though the realm stayed empty as the user
-- manifest was never actually synced).
--
-- The outcome is of this is any Outsider that used Parsec < 3.3.0 has created
-- a realm that is considered illegal in Parsec >= 3.3.0 :/
--
-- Hence this migration that solve this issue by simply removing any realm
-- created by an Outsider.
-- This is fine since no other user knows about this realm, and the Outsider
-- is not allowed to access it anyway.
-------------------------------------------------------

WITH to_remove_realm AS (
    SELECT realm_user_role.realm
    FROM realm_user_role
    WHERE
        -- Only keep the roles that correspond to a realm creation (i.e. the user gives the role to himself)
        realm_user_role.user_ = (
            SELECT device.user_
            FROM device
            WHERE device._id = realm_user_role.certified_by
        )
        AND
        -- Find the profile the user had when this role was created
        COALESCE(
            (
                SELECT profile.profile
                FROM profile
                WHERE
                    profile.user_ = realm_user_role.user_
                    AND profile.certified_on <= realm_user_role.certified_on
                ORDER BY profile.certified_on DESC LIMIT 1
            ),
            -- The user profile has never been updated, use the initial one
            (
                SELECT user_.initial_profile
                FROM user_
                WHERE user_._id = realm_user_role.user_
            )
        ) = 'OUTSIDER'
),

-- Clean all tables that are related to the realm

clean_realm_user_role AS ( -- noqa: ST03
    DELETE FROM realm_user_role
    USING to_remove_realm
    WHERE realm_user_role.realm = to_remove_realm.realm
),

clean_realm_archiving AS ( -- noqa: ST03
    DELETE FROM realm_archiving
    USING to_remove_realm
    WHERE realm_archiving.realm = to_remove_realm.realm
),

clean_realm_user_change AS ( -- noqa: ST03
    DELETE FROM realm_user_change
    USING to_remove_realm
    WHERE realm_user_change.realm = to_remove_realm.realm
),

clean_realm_keys_bundle AS ( -- noqa: ST03
    DELETE FROM realm_keys_bundle
    USING to_remove_realm
    WHERE realm_keys_bundle.realm = to_remove_realm.realm
),

clean_realm_keys_bundle_access AS ( -- noqa: ST03
    DELETE FROM realm_keys_bundle_access
    USING to_remove_realm
    WHERE realm_keys_bundle_access.realm = to_remove_realm.realm
),

clean_realm_sequester_keys_bundle_access AS ( -- noqa: ST03
    DELETE FROM realm_sequester_keys_bundle_access
    USING to_remove_realm
    WHERE realm_sequester_keys_bundle_access.realm = to_remove_realm.realm
),

clean_realm_name AS ( -- noqa: ST03
    DELETE FROM realm_name
    USING to_remove_realm
    WHERE realm_name.realm = to_remove_realm.realm
),

clean_vlob_atom AS ( -- noqa: ST03
    DELETE FROM vlob_atom
    USING to_remove_realm
    WHERE vlob_atom.realm = to_remove_realm.realm
),

clean_realm_vlob_update AS ( -- noqa: ST03
    DELETE FROM realm_vlob_update
    USING to_remove_realm
    WHERE realm_vlob_update.realm = to_remove_realm.realm
),

clean_block AS ( -- noqa: ST03
    DELETE FROM block
    USING to_remove_realm
    WHERE block.realm = to_remove_realm.realm
),

clean_realm_topic AS ( -- noqa: ST03
    DELETE FROM realm_topic
    USING to_remove_realm
    WHERE realm_topic.realm = to_remove_realm.realm
)

-- Clean the realm table
DELETE FROM realm
USING to_remove_realm
WHERE realm._id = to_remove_realm.realm;
