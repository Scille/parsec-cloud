-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Track the effective archiving/deletion status directly on each realm.
-------------------------------------------------------

CREATE TYPE REALM_STATUS AS ENUM ('AVAILABLE', 'ARCHIVED_OR_DELETION_PLANNED', 'DELETED');

ALTER TABLE realm ADD COLUMN status REALM_STATUS NOT NULL DEFAULT 'AVAILABLE';

WITH latest_realm_archiving AS (
    SELECT DISTINCT ON (realm)
        realm,
        configuration
    FROM realm_archiving
    ORDER BY realm ASC, certified_on DESC, _id DESC
)

UPDATE realm
SET
    status = CASE
        WHEN
            latest_realm_archiving.configuration IN ('ARCHIVED', 'DELETION_PLANNED')
            THEN 'ARCHIVED_OR_DELETION_PLANNED'::REALM_STATUS
        ELSE 'AVAILABLE'::REALM_STATUS
    END
FROM latest_realm_archiving
WHERE realm._id = latest_realm_archiving.realm;
