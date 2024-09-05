-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    -- NULL if the organization was spontaneously created (i.e. as part
    -- of organization bootstrap)
    bootstrap_token VARCHAR(32),
    -- NULL if not bootstrapped
    root_verify_key BYTEA,
    -- NULL if not expired
    _expired_on TIMESTAMPTZ,
    user_profile_outsider_allowed BOOLEAN NOT NULL,
    -- NULL if no limit
    active_users_limit INTEGER,
    is_expired BOOLEAN NOT NULL,
    -- NULL if not bootstrapped
    _bootstrapped_on TIMESTAMPTZ,
    _created_on TIMESTAMPTZ NOT NULL,
    -- NULL for non-sequestered organization
    sequester_authority_certificate BYTEA,
    -- NULL for non-sequestered organization
    sequester_authority_verify_key_der BYTEA,
    minimum_archiving_period INTEGER NOT NULL
);

-------------------------------------------------------
-- Sequester
-------------------------------------------------------
CREATE TYPE sequester_service_type AS ENUM ('STORAGE', 'WEBHOOK');

CREATE TABLE sequester_service (
    _id SERIAL PRIMARY KEY,
    service_id UUID NOT NULL,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    service_certificate BYTEA NOT NULL,
    service_label VARCHAR(254) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    disabled_on TIMESTAMPTZ, -- NULL if currently enabled
    webhook_url TEXT, -- NULL if service_type != WEBHOOK;
    service_type SEQUESTER_SERVICE_TYPE NOT NULL,


    revoked_on TIMESTAMPTZ, -- NULL if not yet revoked
    revoked_sequester_certificate BYTEA, -- NULL if not yet revoked
    revoked_sequester_certifier INTEGER, -- NULL if not yet revoked

    UNIQUE (organization, service_id)
);

-------------------------------------------------------
--  User
-------------------------------------------------------


CREATE TABLE human (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    email VARCHAR(254) NOT NULL,
    label VARCHAR(254) NOT NULL,

    UNIQUE (organization, email)
);


CREATE TYPE user_profile AS ENUM ('ADMIN', 'STANDARD', 'OUTSIDER');


CREATE TABLE user_ (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_id UUID NOT NULL,
    user_certificate BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    user_certifier INTEGER,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not yet revoked
    revoked_on TIMESTAMPTZ,
    -- NULL if not yet revoked
    revoked_user_certificate BYTEA,
    -- NULL if not yet revoked
    revoked_user_certifier INTEGER,
    human INTEGER REFERENCES human (_id) NOT NULL,
    redacted_user_certificate BYTEA NOT NULL,
    initial_profile USER_PROFILE NOT NULL,
    -- This field is altered in an `ALTER TABLE` statement below
    -- in order to avoid cross-reference issues
    shamir_recovery INTEGER,
    frozen BOOLEAN NOT NULL DEFAULT FALSE,
    current_profile USER_PROFILE NOT NULL,

    UNIQUE (organization, user_id)
);

CREATE TABLE profile (
    _id SERIAL PRIMARY KEY,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    profile USER_PROFILE NOT NULL,
    profile_certificate BYTEA NOT NULL,
    certified_by INTEGER NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


CREATE TABLE device (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    device_id UUID NOT NULL,
    device_label VARCHAR(254) NOT NULL,
    verify_key BYTEA NOT NULL,
    device_certificate BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    device_certifier INTEGER REFERENCES device (_id),
    created_on TIMESTAMPTZ NOT NULL,
    redacted_device_certificate BYTEA NOT NULL,

    UNIQUE (organization, device_id)
);


ALTER TABLE user_
ADD CONSTRAINT fk_user_device_user_certifier FOREIGN KEY (
    user_certifier
) REFERENCES device (_id);
ALTER TABLE user_
ADD CONSTRAINT fk_user_device_revoked_user_certifier FOREIGN KEY (
    revoked_user_certifier
) REFERENCES device (_id);
ALTER TABLE sequester_service ADD FOREIGN KEY (
    revoked_sequester_certifier
) REFERENCES device (_id);

ALTER TABLE profile ADD FOREIGN KEY (certified_by) REFERENCES device (_id);

-------------------------------------------------------
--  Shamir recovery
-------------------------------------------------------


CREATE TABLE shamir_recovery_setup (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,

    brief_certificate BYTEA NOT NULL,
    reveal_token UUID NOT NULL,
    threshold INTEGER NOT NULL,
    shares INTEGER NOT NULL,
    ciphered_data BYTEA,

    UNIQUE (organization, reveal_token)
);


CREATE TABLE shamir_recovery_share (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,

    shamir_recovery INTEGER REFERENCES shamir_recovery_setup (_id) NOT NULL,
    recipient INTEGER REFERENCES user_ (_id) NOT NULL,

    share_certificate BYTEA NOT NULL,
    shares INTEGER NOT NULL,

    UNIQUE (organization, shamir_recovery, recipient)
);



-- Alter user table to introduce a cross-reference between user id and shamir id
ALTER TABLE user_ ADD FOREIGN KEY (
    shamir_recovery
) REFERENCES shamir_recovery_setup (_id);


-------------------------------------------------------
--  Invitation
-------------------------------------------------------

CREATE TYPE invitation_type AS ENUM ('USER', 'DEVICE', 'SHAMIR_RECOVERY');
CREATE TYPE invitation_deleted_reason AS ENUM (
    'FINISHED', 'CANCELLED', 'ROTTEN'
);

CREATE TYPE cancelled_greeting_attempt_reason AS ENUM (
    'MANUALLY_CANCELLED',
    'INVALID_NONCE_HASH',
    'INVALID_SAS_CODE',
    'UNDECIPHERABLE_PAYLOAD',
    'UNDESERIALIZABLE_PAYLOAD',
    'INCONSISTENT_PAYLOAD',
    'AUTOMATICALLY_CANCELLED'
);

CREATE TYPE greeter_or_claimer AS ENUM (
    'GREETER',
    'CLAIMER'
);


CREATE TABLE invitation (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    token VARCHAR(32) NOT NULL,
    type INVITATION_TYPE NOT NULL,

    created_by INTEGER REFERENCES device (_id) NOT NULL,
    -- Required for when type=USER
    claimer_email VARCHAR(255),

    created_on TIMESTAMPTZ NOT NULL,
    deleted_on TIMESTAMPTZ,
    deleted_reason INVITATION_DELETED_REASON,

    -- Required for when type=SHAMIR_RECOVERY
    shamir_recovery INTEGER REFERENCES shamir_recovery_setup (_id),

    UNIQUE (organization, token)
);

CREATE TABLE greeting_session (
    _id SERIAL PRIMARY KEY,
    invitation INTEGER REFERENCES invitation (_id) NOT NULL,
    greeter INTEGER REFERENCES user_ (_id) NOT NULL,

    UNIQUE (invitation, greeter)
);

CREATE TABLE greeting_attempt (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    greeting_attempt_id UUID NOT NULL,
    greeting_session INTEGER REFERENCES greeting_session (_id) NOT NULL,

    claimer_joined TIMESTAMPTZ DEFAULT NULL,
    greeter_joined TIMESTAMPTZ DEFAULT NULL,

    cancelled_reason CANCELLED_GREETING_ATTEMPT_REASON DEFAULT NULL,
    cancelled_on TIMESTAMPTZ DEFAULT NULL,
    cancelled_by GREETER_OR_CLAIMER DEFAULT NULL,

    UNIQUE (organization, greeting_attempt_id)
);

-- Makes sure that there is only one active greeting attempt per session
CREATE UNIQUE INDEX unique_active_attempt ON greeting_attempt (greeting_session)
WHERE cancelled_on IS NULL;

CREATE TABLE greeting_step (
    _id SERIAL PRIMARY KEY,
    greeting_attempt INTEGER REFERENCES greeting_attempt (_id) NOT NULL,
    step INTEGER NOT NULL,
    greeter_data BYTEA,
    claimer_data BYTEA,

    UNIQUE (greeting_attempt, step)
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

CREATE TYPE pki_enrollment_info_accepted AS (
    accepted_on TIMESTAMPTZ,
    accepter_der_x509_certificate BYTEA,
    accept_payload_signature BYTEA,
    accept_payload BYTEA
);

CREATE TYPE pki_enrollment_info_rejected AS (
    rejected_on TIMESTAMPTZ
);

CREATE TYPE pki_enrollment_info_cancelled AS (
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

    enrollment_state ENROLLMENT_STATE NOT NULL DEFAULT 'SUBMITTED',
    info_accepted PKI_ENROLLMENT_INFO_ACCEPTED,
    info_rejected PKI_ENROLLMENT_INFO_REJECTED,
    info_cancelled PKI_ENROLLMENT_INFO_CANCELLED,

    UNIQUE (organization, enrollment_id)
);

-------------------------------------------------------
--  Realm
-------------------------------------------------------


CREATE TYPE maintenance_type AS ENUM ('REENCRYPTION', 'GARBAGE_COLLECTION');


CREATE TABLE realm (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm_id UUID NOT NULL,
    key_index INTEGER NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE (organization, realm_id)
);


CREATE TYPE realm_role AS ENUM ('OWNER', 'MANAGER', 'CONTRIBUTOR', 'READER');


CREATE TABLE realm_user_role (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- NULL if access revocation
    role REALM_ROLE,
    certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device (_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);

CREATE TYPE realm_archiving_configuration AS ENUM (
    'AVAILABLE', 'ARCHIVED', 'DELETION_PLANNED'
);

CREATE TABLE realm_archiving (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    configuration REALM_ARCHIVING_CONFIGURATION NOT NULL,
    -- NULL if not DELETION_PLANNED
    deletion_date TIMESTAMPTZ,
    certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device (_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


-- TODO: Investigate which of those timestamp is really needed
CREATE TABLE realm_user_change (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    -- The last time this user changed the role of another user
    last_role_change TIMESTAMPTZ,
    -- The last time this user updated a vlob
    last_vlob_update TIMESTAMPTZ,
    -- The last time this user changed the archiving configuration
    last_archiving_change TIMESTAMPTZ,

    UNIQUE (realm, user_)
);

CREATE TABLE realm_keys_bundle (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    key_index INTEGER NOT NULL,

    realm_key_rotation_certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device (_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL,
    key_canary BYTEA NOT NULL,
    keys_bundle BYTEA NOT NULL,

    UNIQUE (realm, key_index)
);

CREATE TABLE realm_keys_bundle_access (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    realm_keys_bundle INTEGER REFERENCES realm_keys_bundle (_id) NOT NULL,

    access BYTEA NOT NULL,

    UNIQUE (realm, user_, realm_keys_bundle)
);


CREATE TABLE realm_sequester_keys_bundle_access (
    _id SERIAL PRIMARY KEY,
    sequester_service INTEGER REFERENCES sequester_service (_id) NOT NULL,
    realm_keys_bundle INTEGER REFERENCES realm_keys_bundle (_id) NOT NULL,

    access BYTEA NOT NULL,

    UNIQUE (sequester_service, realm_keys_bundle)
);

CREATE TABLE realm_name (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    realm_name_certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device (_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


-------------------------------------------------------
--  Vlob
-------------------------------------------------------

CREATE TABLE vlob_atom (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    key_index INTEGER NOT NULL,
    vlob_id UUID NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    size INTEGER NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ,

    UNIQUE (realm, vlob_id, version)
);


CREATE TABLE realm_vlob_update (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    index INTEGER NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,

    UNIQUE (realm, index)
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
    key_index INTEGER NOT NULL,

    UNIQUE (organization, block_id)
);


-- Only used if we store blocks' data in database
CREATE TABLE block_data (
    _id SERIAL PRIMARY KEY,
    -- No reference with organization&block tables given this table
    -- should stay isolated
    organization_id VARCHAR NOT NULL,
    block_id UUID NOT NULL,
    data BYTEA NOT NULL,

    UNIQUE (organization_id, block_id)
);


-------------------------------------------------------
-- Topic
-------------------------------------------------------

CREATE TABLE common_topic (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE (organization)

);

CREATE TABLE sequester_topic (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE (organization)
);

CREATE TABLE shamir_recovery_topic (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE (organization)
);

CREATE TABLE realm_topic (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE (organization, realm)
);


-------------------------------------------------------
--  Migration
-------------------------------------------------------


CREATE TABLE migration (
    _id INTEGER PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    applied TIMESTAMPTZ NOT NULL
);
