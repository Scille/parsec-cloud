-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

-------------------------------------------------------
-- Organization
-------------------------------------------------------

ALTER TABLE organization ALTER COLUMN bootstrap_token DROP NOT NULL;


-------------------------------------------------------
-- User
-------------------------------------------------------

ALTER TABLE user_ RENAME COLUMN profile TO initial_profile;

CREATE TABLE profile (
    _id SERIAL PRIMARY KEY,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    profile user_profile NOT NULL,
    profile_certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device(_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


-------------------------------------------------------
-- Sequester
-------------------------------------------------------

-- Add sequester service revocation
ALTER TABLE sequester_service ADD revoked_on TIMESTAMPTZ;
ALTER TABLE sequester_service ADD revoked_sequester_certificate BYTEA;
ALTER TABLE sequester_service ADD revoked_sequester_certifier INTEGER REFERENCES device(_id);

-------------------------------------------------------
--  Invitation
-------------------------------------------------------

-- Rename invitation greeter to invitation author
ALTER TABLE invitation RENAME COLUMN greeter TO author;
ALTER TABLE invitation RENAME CONSTRAINT invitation_greeter_fkey TO invitation_author_fkey;

-- Remove conduit columns from invitation table
ALTER TABLE invitation DROP COLUMN conduit_state;
ALTER TABLE invitation DROP COLUMN conduit_greeter_payload;
ALTER TABLE invitation DROP COLUMN conduit_claimer_payload;

-- Rename shamir_recovery_conduit to invitation_conduit
ALTER TABLE shamir_recovery_conduit RENAME TO invitation_conduit;
ALTER SEQUENCE shamir_recovery_conduit__id_seq RENAME TO invitation_conduit__id_seq;
ALTER TABLE invitation_conduit RENAME CONSTRAINT shamir_recovery_conduit_pkey TO invitation_conduit_pkey;
ALTER TABLE invitation_conduit RENAME CONSTRAINT shamir_recovery_conduit_invitation_greeter_key TO invitation_conduit_invitation_greeter_key;
ALTER TABLE invitation_conduit RENAME CONSTRAINT shamir_recovery_conduit_greeter_fkey TO invitation_conduit_greeter_fkey;
ALTER TABLE invitation_conduit RENAME CONSTRAINT shamir_recovery_conduit_invitation_fkey TO invitation_conduit_invitation_fkey;

ALTER TABLE invitation_conduit ADD last_exchange BOOLEAN;

-------------------------------------------------------
--  Message
-------------------------------------------------------

-- Remove message table
DROP TABLE message;

-------------------------------------------------------
--  Realm
-------------------------------------------------------

-- Rename encryption_revision to key_index
ALTER TABLE realm RENAME encryption_revision TO key_index;
ALTER TABLE realm ALTER COLUMN key_index TYPE INTEGER;

-- Remove maintenance column from realm table
ALTER TABLE realm DROP COLUMN maintenance_started_by;
ALTER TABLE realm DROP COLUMN maintenance_started_on;
ALTER TABLE realm DROP COLUMN maintenance_type;

-- Add created_on column to realm table
ALTER TABLE realm ADD created_on TIMESTAMPTZ NOT NULL;

-- Add realm_keys_bundle table
create TABLE realm_keys_bundle (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    key_index INTEGER NOT NULL,

    realm_key_rotation_certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device(_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL,
    key_canary BYTEA NOT NULL,
    keys_bundle BYTEA NOT NULL,

    UNIQUE(realm, key_index)
);

-- Add realm_keys_bundle_access table
create TABLE realm_keys_bundle_access (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    realm_keys_bundle INTEGER REFERENCES realm_keys_bundle (_id) NOT NULL,

    access BYTEA NOT NULL,

    UNIQUE(realm, user_, realm_keys_bundle)
);

-- Add realm_sequester_keys_bundle_access table
create TABLE realm_sequester_keys_bundle_access (
    _id SERIAL PRIMARY KEY,
    sequester_service INTEGER REFERENCES sequester_service (_id) NOT NULL,
    realm_keys_bundle INTEGER REFERENCES realm_keys_bundle (_id) NOT NULL,

    access BYTEA NOT NULL,

    UNIQUE(sequester_service, realm_keys_bundle)
);

-- Add realm_name table
create TABLE realm_name (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    realm_name_certificate BYTEA NOT NULL,
    certified_by INTEGER REFERENCES device(_id) NOT NULL,
    certified_on TIMESTAMPTZ NOT NULL
);


-------------------------------------------------------
--  Vlob
-------------------------------------------------------

-- Rename vlob_encryption_revision to key_index
ALTER TABLE vlob_atom RENAME organization TO realm;
ALTER TABLE vlob_atom DROP CONSTRAINT vlob_atom_organization_fkey;
ALTER TABLE vlob_atom ADD CONSTRAINT vlob_atom_realm_fkey FOREIGN KEY (realm) REFERENCES realm(_id);


-- Rename vlob_encryption_revision to key_index
ALTER TABLE vlob_atom RENAME vlob_encryption_revision TO key_index;
ALTER TABLE vlob_atom ALTER COLUMN key_index TYPE INTEGER;
ALTER TABLE vlob_atom DROP CONSTRAINT vlob_atom_vlob_encryption_revision_fkey;
ALTER TABLE vlob_atom DROP CONSTRAINT vlob_atom_vlob_encryption_revision_vlob_id_version_key;

-- Update unique contraint
ALTER TABLE vlob_atom ADD CONSTRAINT vlob_atom_realm_vlob_id_version_key UNIQUE (realm, vlob_id, version);

-- Remove vlob_encryption_revision table
ALTER TABLE vlob_encryption_revision DROP CONSTRAINT vlob_encryption_revision_realm_fkey;
DROP TABLE vlob_encryption_revision;

-- Remove sequester_service_vlob_atom table
DROP TABLE sequester_service_vlob_atom;


-------------------------------------------------------
-- Block
-------------------------------------------------------

ALTER TABLE block ADD key_index INTEGER NOT NULL;

-------------------------------------------------------
-- Topic
-------------------------------------------------------

CREATE TABLE topic_common (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(organization)

);

CREATE TABLE topic_sequester (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(organization)
);

CREATE TABLE topic_shamir_recovery (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(organization)
);

CREATE TABLE topic_realm (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    last_timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(organization, realm)
);
