-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

DO $$
BEGIN
    -- Realms that should have been kept: 1, 2, 5, 6

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 1
   ), 'Realm 1 should have been kept';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 2
   ), 'Realm 2 should have been kept';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 5
   ), 'Realm 5 should have been kept';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 6
   ), 'Realm 6 should have been kept';

    -- Realms that should have been removed: 3, 4, 7, 8, 9

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 3
   ), 'Realm 3 should has been removed';

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 4
   ), 'Realm 4 should has been removed';

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 7
   ), 'Realm 7 should has been removed';

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 8
   ), 'Realm 8 should has been removed';

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm
      WHERE
        _id = 9
   ), 'Realm 9 should has been removed';

   -- Check block table

   ASSERT NOT EXISTS (
      SELECT *
      FROM block
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some row in the block table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM block
      WHERE realm IN (1, 2, 5, 6)
   ) = 4, 'Some row in the block table has been incorrectly removed';

   -- Check realm_keys_bundle table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_keys_bundle
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_keys_bundle table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_keys_bundle
      WHERE realm IN (1, 2, 5, 6)
      -- 5 because realm 2 got 2 key rotation
   ) = 5, 'Some rows in the realm_keys_bundle table has been incorrectly removed';

   -- Check realm_user_role table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_user_role
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_user_role table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_user_role
      WHERE realm IN (1, 2, 5, 6)
      -- 5 because realm 2 got shared once
   ) = 5, 'Some rows in the realm_user_role table has been incorrectly removed';

   -- Check realm_keys_bundle_access table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_keys_bundle_access
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_keys_bundle_access table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_keys_bundle_access
      WHERE realm IN (1, 2, 5, 6)
      -- 7 because realm 2 is shared once, then got a key rotation
   ) = 7, 'Some rows in the realm_keys_bundle_access table has been incorrectly removed';

   -- Check realm_name table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_name
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_name table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_name
      WHERE realm IN (1, 2, 5, 6)
   ) = 4, 'Some rows in the realm_name table has been incorrectly removed';

   -- Check realm_topic table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_topic
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_topic table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_topic
      WHERE realm IN (1, 2, 5, 6)
   ) = 4, 'Some rows in the realm_topic table has been incorrectly removed';

   -- Check vlob_atom table

   ASSERT NOT EXISTS (
      SELECT *
      FROM vlob_atom
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the vlob_atom table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM vlob_atom
      WHERE realm IN (1, 2, 5, 6)
   ) = 4, 'Some rows in the vlob_atom table has been incorrectly removed';

   -- Check realm_vlob_update table

   ASSERT NOT EXISTS (
      SELECT *
      FROM realm_vlob_update
      WHERE realm IN (3, 4, 7, 8, 9)
   ), 'Some rows in the realm_vlob_update table has been incorrectly kept';

   ASSERT (
      SELECT COUNT(*)
      FROM realm_vlob_update
      WHERE realm IN (1, 2, 5, 6)
   ) = 4, 'Some rows in the realm_vlob_update table has been incorrectly removed';

END$$;
