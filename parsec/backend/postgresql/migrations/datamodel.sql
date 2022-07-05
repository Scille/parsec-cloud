-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


-- This SQL script is not actually used to initialize the database: we rely
-- on all the migrations scripts for that (even if the database is brand new !).
-- The true role of this script is to provide a unified and up to date view
-- of the current datamodel.
-- On top of that we test the migrations and this script against each other to
-- ensure consistency.


-------------------------------------------------------
--  Organization
-------------------------------------------------------


CREATE TABLE organization (
    _id SERIAL PRIMARY KEY,
    organization_id VARCHAR(32) UNIQUE NOT NULL,
    bootstrap_token TEXT NOT NULL,
    root_verify_key BYTEA,
    _expired_on TIMESTAMPTZ,
    user_profile_outsider_allowed BOOLEAN NOT NULL,
    active_users_limit INTEGER,
    is_expired BOOLEAN NOT NULL,
    _bootstrapped_on TIMESTAMPTZ,
    _created_on TIMESTAMPTZ NOT NULL
    sequester_authority BYTEA; -- NULL To disable sequester services
);

-------------------------------------------------------
-- Sequester
-------------------------------------------------------

CREATE TABLE sequester_service(
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    service_certificate BYTEA NOT NULL,
    service_label VARCHAR(254) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    deleted_on TIMESTAMPTZ, -- NULL if not deleted
    webhook_url TEXT, -- NULL if service is not a WEBHOOK

    UNIQUE(organization, service_id)
);

-------------------------------------------------------
--  User
-------------------------------------------------------


CREATE TABLE human (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    email VARCHAR(254) NOT NULL,
    label VARCHAR(254) NOT NULL,

    UNIQUE(organization, email)
);


CREATE TYPE user_profile AS ENUM ('ADMIN', 'STANDARD', 'OUTSIDER');


CREATE TABLE user_ (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_id VARCHAR(32) NOT NULL,
    user_certificate BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    user_certifier INTEGER,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not yet revoked
    revoked_on TIMESTAMPTZ,
    -- NULL if not yet revoked
    revoked_user_certificate BYTEA,
    -- NULL if certifier is the Root Verify Key
    revoked_user_certifier INTEGER,
    human INTEGER REFERENCES human (_id),
    redacted_user_certificate BYTEA NOT NULL,
    profile user_profile NOT NULL,

    UNIQUE(organization, user_id)
);


CREATE TABLE device (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    device_id VARCHAR(65) NOT NULL,
    device_certificate BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    device_certifier INTEGER REFERENCES device (_id),
    created_on TIMESTAMPTZ NOT NULL,
    redacted_device_certificate BYTEA NOT NULL,
    device_label VARCHAR(254),

    UNIQUE(organization, device_id),
    UNIQUE(user_, device_id)
);


ALTER TABLE user_
ADD CONSTRAINT FK_user_device_user_certifier FOREIGN KEY (user_certifier) REFERENCES device (_id);
ALTER TABLE user_
ADD CONSTRAINT FK_user_device_revoked_user_certifier FOREIGN KEY (revoked_user_certifier) REFERENCES device (_id);


CREATE TYPE invitation_type AS ENUM ('USER', 'DEVICE');
CREATE TYPE invitation_deleted_reason AS ENUM ('FINISHED', 'CANCELLED', 'ROTTEN');
CREATE TYPE invitation_conduit_state AS ENUM (
    '1_WAIT_PEERS',
    '2_1_CLAIMER_HASHED_NONCE',
    '2_2_GREETER_NONCE',
    '2_3_CLAIMER_NONCE',
    '3_1_CLAIMER_TRUST',
    '3_2_GREETER_TRUST',
    '4_COMMUNICATE'
);


CREATE TABLE invitation (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    token UUID NOT NULL,
    type invitation_type NOT NULL,

    greeter INTEGER REFERENCES user_ (_id) NOT NULL,
    -- greeter_human INTEGER REFERENCES human (_id),
    claimer_email VARCHAR(255),  -- Required for when type=USER
    created_on TIMESTAMPTZ NOT NULL,

    deleted_on TIMESTAMPTZ,
    deleted_reason invitation_deleted_reason,

    conduit_state invitation_conduit_state NOT NULL DEFAULT '1_WAIT_PEERS',
    conduit_greeter_payload BYTEA,
    conduit_claimer_payload BYTEA,

    UNIQUE(organization, token)
);


-------------------------------------------------------
--  PKI enrollment
-------------------------------------------------------


CREATE TYPE enrollment_state AS ENUM (
    'SUBMITTED',
    'ACCEPTED',
    'REJECTED',
    'CANCELLED'
);

CREATE TYPE pki_enrollment_info_accepted AS(
    accepted_on TIMESTAMPTZ,
    accepter_der_x509_certificate BYTEA,
    accept_payload_signature BYTEA,
    accept_payload BYTEA
);

CREATE TYPE pki_enrollment_info_rejected AS(
    rejected_on TIMESTAMPTZ
);

CREATE TYPE pki_enrollment_info_cancelled AS(
    cancelled_on TIMESTAMPTZ
);

CREATE TABLE pki_enrollment (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,

    enrollment_id UUID NOT NULL,
    submitter_der_x509_certificate BYTEA NOT NULL,
    submitter_der_x509_certificate_sha1 BYTEA NOT NULL,

    submit_payload_signature BYTEA NOT NULL,
    submit_payload BYTEA NOT NULL,
    submitted_on TIMESTAMPTZ NOT NULL,

    accepter INTEGER REFERENCES device (_id),
    submitter_accepted_device INTEGER REFERENCES device (_id),

    enrollment_state enrollment_state NOT NULL DEFAULT 'SUBMITTED',
    info_accepted pki_enrollment_info_accepted,
    info_rejected pki_enrollment_info_rejected,
    info_cancelled pki_enrollment_info_cancelled,

    UNIQUE(organization, enrollment_id)
);


-------------------------------------------------------
--  Message
-------------------------------------------------------


CREATE TABLE message (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    recipient INTEGER REFERENCES user_ (_id) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    index INTEGER NOT NULL,
    sender INTEGER REFERENCES device (_id) NOT NULL,
    body BYTEA NOT NULL
);


-------------------------------------------------------
--  Realm
-------------------------------------------------------


CREATE TYPE maintenance_type AS ENUM ('REENCRYPTION', 'GARBAGE_COLLECTION');


CREATE TABLE realm (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm_id UUID NOT NULL,
    encryption_revision INTEGER NOT NULL,
    -- NULL if not currently in maintenance
    maintenance_started_by INTEGER REFERENCES device (_id),
    maintenance_started_on TIMESTAMPTZ,
    maintenance_type maintenance_type,

    UNIQUE(organization, realm_id)
);


CREATE TYPE realm_role AS ENUM ('OWNER', 'MANAGER', 'CONTRIBUTOR', 'READER');


CREATE TABLE realm_user_role (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- NULL if access revocation
    role realm_role,
    certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device(_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


CREATE TABLE realm_user_change (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- The last time this user changed the role of another user
    last_role_change TIMESTAMPTZ,
    -- The last time this user updated a vlob
    last_vlob_update TIMESTAMPTZ,

    UNIQUE(realm, user_)
);


-------------------------------------------------------
--  Vlob
-------------------------------------------------------


CREATE TABLE vlob_encryption_revision (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id),
    encryption_revision INTEGER NOT NULL,

    UNIQUE(realm, encryption_revision)
);


CREATE TABLE vlob_atom (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    vlob_encryption_revision INTEGER REFERENCES vlob_encryption_revision (_id) NOT NULL,
    vlob_id UUID NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    size INTEGER NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ,

    UNIQUE(vlob_encryption_revision, vlob_id, version)
);


CREATE TABLE realm_vlob_update (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    index INTEGER NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,

    UNIQUE(realm, index)
);


CREATE TABLE sequester_service_vlob(
    _id SERIAL PRIMARY KEY,
    vlob INTEGER REFERENCES vlob_atom (_id) NOT NULL,
    service INTEGER REFERENCES sequester_service (_id) NOT NULL,
    blob BYTEA NOT NULL
);

-------------------------------------------------------
--  Block
-------------------------------------------------------


CREATE TABLE block (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    block_id UUID NOT NULL,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    size INTEGER NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ,

    UNIQUE(organization, block_id)
);


-- Only used if we store blocks' data in database
CREATE TABLE block_data (
    _id SERIAL PRIMARY KEY,
    -- No reference with organization&block tables given this table
    -- should stay isolated
    organization_id VARCHAR NOT NULL,
    block_id UUID NOT NULL,
    data BYTEA NOT NULL,

    UNIQUE(organization_id, block_id)
);


-------------------------------------------------------
--  Migration
-------------------------------------------------------


CREATE TABLE migration (
    _id INTEGER PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    applied TIMESTAMPTZ NOT NULL
);
