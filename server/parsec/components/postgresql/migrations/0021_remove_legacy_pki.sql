-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Remove tables and types related to legacy pki
-------------------------------------------------------
DROP TABLE pki_enrollment;
DROP TABLE pki_certificate;

DROP TYPE ENROLLMENT_STATE;
DROP TYPE PKI_ENROLLMENT_INFO_ACCEPTED;
DROP TYPE PKI_ENROLLMENT_INFO_REJECTED;
DROP TYPE PKI_ENROLLMENT_INFO_CANCELLED;
