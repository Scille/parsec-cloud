-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

DO $$
DECLARE
   organization_internal_id integer;
   user_current_profile text;
BEGIN
   SELECT _id
   INTO organization_internal_id
   FROM organization
   WHERE organization_id = 'Org1';

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'alice@example.com';

   ASSERT user_current_profile = 'ADMIN', FORMAT('Bad current profile for Alice: `%s`', user_current_profile);

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'bob@example.com';

   ASSERT user_current_profile = 'STANDARD', FORMAT('Bad current profile for Bob: `%s`', user_current_profile);

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'mallory@example.com';

   ASSERT user_current_profile = 'OUTSIDER', FORMAT('Bad current profile for Mallory: `%s`', user_current_profile);

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'zack@invalid.com';

   ASSERT user_current_profile = 'ADMIN', FORMAT('Bad current profile for Zack: `%s`', user_current_profile);

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'marty@invalid.com';

   ASSERT user_current_profile = 'STANDARD', FORMAT('Bad current profile for Marty: `%s`', user_current_profile);

   SELECT user_.current_profile
   INTO user_current_profile
   FROM user_
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      user_.organization = organization_internal_id
      AND human.email = 'doc@invalid.com';

   ASSERT user_current_profile = 'ADMIN', FORMAT('Bad current profile for Doc: `%s`', user_current_profile);
END$$;
