-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Remove column `submitter_der_x509_certificate_sha1` to the `pki_enrollment` table
-------------------------------------------------------

ALTER TABLE pki_enrollment DROP submitter_der_x509_certificate_sha1;
