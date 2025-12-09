-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Add the table `certificate` to store the PKI x509 certificate linked together
-------------------------------------------------------


CREATE TABLE pki_certificate (
    sha256_fingerprint BYTEA PRIMARY KEY,
    -- Fingerprint of a certificate that signed this certificate
    -- Maybe null if not signed by an other known certificate or self-signed.
    signed_by BYTEA REFERENCES pki_certificate (sha256_fingerprint),
    -- The DER content of the certificate
    der_content BYTEA NOT NULL
);
