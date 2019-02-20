-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

CREATE TABLE organizations (
    _id SERIAL PRIMARY KEY,
    organization_id VARCHAR(32) UNIQUE NOT NULL,
    bootstrap_token TEXT NOT NULL,
    root_verify_key BYTEA
);


CREATE FUNCTION get_organization_internal_id(VARCHAR) RETURNS integer AS $$
DECLARE
BEGIN
    RETURN (
        SELECT _id from organizations WHERE organization_id = $1
    );
END;
$$ LANGUAGE plpgsql STABLE;


CREATE FUNCTION get_user_internal_id(orgid VARCHAR, userid VARCHAR) RETURNS integer AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM users
        WHERE
            organization = get_organization_internal_id(orgid)
            AND user_id = userid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_device_internal_id(orgid VARCHAR, deviceid VARCHAR) RETURNS integer AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM devices
        WHERE
            organization = get_organization_internal_id(orgid)
            AND device_id = deviceid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_device_id(deviceinternalid INTEGER) RETURNS VARCHAR AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            device_id
        FROM devices
        WHERE
            _id = deviceinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE TABLE users (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    user_id VARCHAR(32) NOT NULL,
    is_admin BOOLEAN NOT NULL,
    certified_user BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    user_certifier INTEGER,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, user_id)
);


CREATE TABLE devices (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    user_ INTEGER REFERENCES users (_id) NOT NULL,
    device_id VARCHAR(65) NOT NULL,
    certified_device BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    device_certifier INTEGER REFERENCES devices (_id),
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not yet revocated
    revocated_on TIMESTAMPTZ,
    -- NULL if not yet revocated
    certified_revocation BYTEA,
    -- NULL if certifier is the Root Verify Key
    revocation_certifier INTEGER REFERENCES devices (_id),

    UNIQUE(organization, device_id),
    UNIQUE(user_, device_id)
);


ALTER TABLE users
ADD CONSTRAINT FK_users_devices FOREIGN KEY (user_certifier) REFERENCES devices (_id);


CREATE TABLE user_invitations (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    user_id VARCHAR(32) NOT NULL,
    creator INTEGER REFERENCES devices (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, user_id)
);


CREATE TABLE device_invitations (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    device_id VARCHAR(65) NOT NULL,
    creator INTEGER REFERENCES devices (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, device_id)
);


CREATE TABLE messages (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    recipient INTEGER REFERENCES users (_id) NOT NULL,
    sender INTEGER REFERENCES devices (_id) NOT NULL,
    body BYTEA NOT NULL
);


CREATE TABLE vlobs (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    vlob_id UUID NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    author INTEGER REFERENCES devices (_id) NOT NULL,
    UNIQUE(organization, vlob_id, version)
);


CREATE TABLE beacons (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    beacon_id UUID NOT NULL,
    beacon_index INTEGER NOT NULL,
    -- src_id UUID NOT NULL,
    src_id INTEGER REFERENCES vlobs (_id) NOT NULL,
    -- src_version INTEGER NOT NULL,
    UNIQUE(organization, beacon_id, beacon_index)
);


CREATE TABLE vlobs_per_beacon (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    vlob INTEGER REFERENCES vlobs (_id),
    beacon INTEGER REFERENCES beacons (_id)
);



CREATE TABLE blockstore (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    block_id UUID UNIQUE NOT NULL,
    block BYTEA NOT NULL,
    author INTEGER REFERENCES devices (_id) NOT NULL
);


-- TODO: continue blockstore rework...

-- CREATE TABLE blocks (
--     _id SERIAL PRIMARY KEY,
--     organization INTEGER REFERENCES organizations (_id) NOT NULL,
--     beacon INTEGER REFERENCES beacons (_id) NOT NULL,
--     block_id UUID UNIQUE NOT NULL,
--     block BYTEA NOT NULL,
--     author INTEGER REFERENCES devices (_id) NOT NULL,
--     deleted BOOLEAN NOT NULL
-- );


-- -- Only used if we store blocks' data in database
-- CREATE TABLE blocks_data (
--     _id SERIAL PRIMARY KEY,
--     data BYTEA NOT NULL,
-- );
