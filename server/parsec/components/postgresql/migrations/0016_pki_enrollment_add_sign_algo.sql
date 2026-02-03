-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add column `submit_payload_signature_algorithm` to the `pki_enrollment` table
-- Add attribute `accept_payload_signature_algorithm` to the `pki_enrollment_info_accepted` type
-------------------------------------------------------

CREATE TYPE PKI_SIGNATURE_ALGORITHM AS ENUM ('RSASSA_PSS_SHA256');

-- Note it's okay to declare the new column as `NOT NULL` given this table has
-- no row since it was never used so far (it has been kept from Parsec v2 in
-- prevision of a future support of PKI enrollment).
ALTER TABLE pki_enrollment ADD submit_payload_signature_algorithm PKI_SIGNATURE_ALGORITHM NOT NULL;
ALTER TYPE PKI_ENROLLMENT_INFO_ACCEPTED ADD ATTRIBUTE accept_payload_signature_algorithm PKI_SIGNATURE_ALGORITHM;
