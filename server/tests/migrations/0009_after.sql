-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

DO $$
DECLARE
   organization_internal_id integer;
   user_current_profile text;
   device_invitation_claimer_human_email text;
BEGIN
   SELECT _id
   INTO organization_internal_id
   FROM organization
   WHERE organization_id = 'Org1';

   -- Ensure that the `device_invitation_claimer` column has been correctly updated
   SELECT human.email
   INTO device_invitation_claimer_human_email
   FROM invitation
   INNER JOIN user_
   ON user_._id = invitation.device_invitation_claimer
   INNER JOIN human
   ON user_.human = human._id
   WHERE
      invitation.organization = organization_internal_id
      AND invitation.token = 'e0000000000000000000000000000002';

   ASSERT device_invitation_claimer_human_email = 'alice@example.com', FORMAT('Bad invitation migration: `%s`', device_invitation_claimer_human_email);
END$$;
