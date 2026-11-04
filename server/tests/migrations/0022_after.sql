-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

DO $$
BEGIN
   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12001 AND status = 'AVAILABLE'
   ), 'Realm 12001 should keep default status when it has no archiving history';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12002 AND status = 'AVAILABLE'
   ), 'Realm 12002 should be AVAILABLE when its latest archiving configuration is AVAILABLE';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12003 AND status = 'ARCHIVED_OR_DELETION_PLANNED'
   ), 'Realm 12003 should be ARCHIVED_OR_DELETION_PLANNED when its latest archiving configuration is ARCHIVED';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12004 AND status = 'ARCHIVED_OR_DELETION_PLANNED'
   ), 'Realm 12004 should be ARCHIVED_OR_DELETION_PLANNED when its latest archiving configuration is DELETION_PLANNED';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12005 AND status = 'AVAILABLE'
   ), 'Realm 12005 should be AVAILABLE when its only archiving configuration is AVAILABLE';

   ASSERT EXISTS (
      SELECT *
      FROM realm
      WHERE _id = 12006 AND status = 'AVAILABLE'
   ), 'Realm 12006 should keep default status when it has no archiving history';
END$$;
