-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
--  Migration
--
-- Async enrollment support
-------------------------------------------------------

-- Note this table only store leaf and intermediate certificates, since the
-- root certificates are supposed to be already known (and trusted!) by the
-- machine doing the certificate validation.
CREATE TABLE pki_x509_certificate (
    sha256_fingerprint BYTEA PRIMARY KEY,
    -- Reference to the issuer certificate.
    -- NULL if the certificate is self-signed or signed by an unknown certificate.
    -- In practice this means any certificate signed by a root certificate is going
    -- to have `signed_by` set to NULL.
    parent BYTEA REFERENCES pki_x509_certificate (sha256_fingerprint),
    der_content BYTEA NOT NULL
);

CREATE TYPE async_enrollment_signature_type AS ENUM ('PKI', 'OPENBAO');

CREATE TYPE async_enrollment_state AS ENUM ('SUBMITTED', 'ACCEPTED', 'REJECTED', 'CANCELLED');

CREATE TABLE async_enrollment (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    enrollment_id UUID NOT NULL,

    submitted_on TIMESTAMPTZ NOT NULL,
    submit_payload BYTEA NOT NULL,
    submit_payload_requested_human_handle_email VARCHAR(254) NOT NULL,
    submit_payload_signature_type ASYNC_ENROLLMENT_SIGNATURE_TYPE NOT NULL,

    -- PKI signature fields (NULL if signature_type is OPENBAO)
    submit_payload_signature_pki_signature BYTEA,
    submit_payload_signature_pki_algorithm PKI_SIGNATURE_ALGORITHM,
    submit_payload_signature_pki_author_x509_certificate BYTEA REFERENCES pki_x509_certificate (sha256_fingerprint),

    -- OpenBao signature fields (NULL if signature_type is PKI)
    submit_payload_signature_openbao_signature TEXT,
    submit_payload_signature_openbao_author_entity_id TEXT,

    state ASYNC_ENROLLMENT_STATE NOT NULL DEFAULT 'SUBMITTED',

    -- Accept fields (NULL if not accepted)
    accepted_on TIMESTAMPTZ,
    accept_payload BYTEA,
    accept_payload_signature_type ASYNC_ENROLLMENT_SIGNATURE_TYPE,
    accept_payload_signature_pki_signature BYTEA,
    accept_payload_signature_pki_algorithm PKI_SIGNATURE_ALGORITHM,
    accept_payload_signature_pki_author_x509_certificate BYTEA REFERENCES pki_x509_certificate (sha256_fingerprint),
    accept_payload_signature_openbao_signature TEXT,
    accept_payload_signature_openbao_author_entity_id TEXT,

    -- Reject/Cancel fields
    rejected_on TIMESTAMPTZ,
    cancelled_on TIMESTAMPTZ,

    UNIQUE (organization, enrollment_id)
);
