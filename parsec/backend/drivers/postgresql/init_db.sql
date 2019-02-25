-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
-- Organization
-------------------------------------------------------


CREATE TABLE organizations (
    _id SERIAL PRIMARY KEY,
    organization_id VARCHAR(32) UNIQUE NOT NULL,
    bootstrap_token TEXT NOT NULL,
    root_verify_key BYTEA
);


CREATE FUNCTION get_organization_internal_id(VARCHAR) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT _id from organizations WHERE organization_id = $1
    );
END;
$$ LANGUAGE plpgsql STABLE;


-------------------------------------------------------
--  User&Device
-------------------------------------------------------


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


CREATE FUNCTION get_user_internal_id(orgid VARCHAR, userid VARCHAR) RETURNS INTEGER AS $$
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


CREATE FUNCTION get_user_id(userinternalid INTEGER) RETURNS VARCHAR AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            user_id
        FROM users
        WHERE _id = userinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_device_internal_id(orgid VARCHAR, deviceid VARCHAR) RETURNS INTEGER AS $$
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


-------------------------------------------------------
--  Message
-------------------------------------------------------


CREATE TABLE messages (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    recipient INTEGER REFERENCES users (_id) NOT NULL,
    index INTEGER,
    sender INTEGER REFERENCES devices (_id) NOT NULL,
    body BYTEA NOT NULL
);


-------------------------------------------------------
--  User&Device invitation
-------------------------------------------------------


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


-------------------------------------------------------
--  Vlob&Beacon
-------------------------------------------------------


CREATE TABLE beacons (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    beacon_id UUID NOT NULL,

    UNIQUE(organization, beacon_id)
);


CREATE TABLE beacon_accesses (
    _id SERIAL PRIMARY KEY,
    beacon INTEGER REFERENCES beacons (_id) NOT NULL,
    user_ INTEGER REFERENCES users (_id) NOT NULL,
    admin_access BOOLEAN NOT NULL,
    read_access BOOLEAN NOT NULL,
    write_access BOOLEAN NOT NULL,

    UNIQUE(beacon, user_)
);


CREATE TABLE vlobs (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organizations (_id) NOT NULL,
    beacon INTEGER REFERENCES beacons (_id),
    vlob_id UUID NOT NULL,

    UNIQUE(organization, vlob_id)
);


CREATE TABLE vlob_atoms (
    _id SERIAL PRIMARY KEY,
    vlob INTEGER REFERENCES vlobs (_id) NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    author INTEGER REFERENCES devices (_id) NOT NULL,
    -- created_on TIMESTAMPTZ NOT NULL, TODO

    UNIQUE(vlob, version)
);


CREATE TABLE beacon_vlob_atom_updates (
    _id SERIAL PRIMARY KEY,
    beacon INTEGER REFERENCES beacons (_id) NOT NULL,
    index INTEGER NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atoms (_id) NOT NULL,

    UNIQUE(beacon, index)
);


CREATE FUNCTION user_can_administrate_beacon(userinternalid INTEGER, beaconinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM beacon_accesses
        WHERE
            beacon = beaconinternalid
            AND user_ = userinternalid
            AND admin_access = TRUE
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_can_read_beacon(userinternalid INTEGER, beaconinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM beacon_accesses
        WHERE
            beacon = beaconinternalid
            AND user_ = userinternalid
            AND read_access = TRUE
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_can_write_beacon(userinternalid INTEGER, beaconinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM beacon_accesses
        WHERE
            beacon = beaconinternalid
            AND user_ = userinternalid
            AND write_access = TRUE
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_beacon_internal_id(orgid VARCHAR, beaconid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM beacons
        WHERE
            organization = get_organization_internal_id(orgid)
            AND beacon_id = beaconid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_internal_id(orgid VARCHAR, vlobid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM vlobs
        WHERE
            organization = get_organization_internal_id(orgid)
            AND vlob_id = vlobid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_id(vlobinternalid INTEGER) RETURNS UUID AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            vlob_id
        FROM vlobs
        WHERE _id = vlobinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


-- CREATE FUNCTION register_vlob_atom_in_beacon(vlob_atom INTEGER) RETURNS integer AS $$
-- DECLARE
-- BEGIN
--     INSERT INTO beacon_vlob_atom_updates(
--         beacon,
--         index,
--         vlob_atom
--     )
--     SELECT
--         (SELECT beacon FROM vlobs WHERE
--             _id = (SELECT vlob FROM vlob_atoms WHERE _id = vlob_atom)
--         ),
--         COUNT(SELECT true FROM beacon_vlob_atom_updates WHERE _id = vlob_atom),
--         vlob_atom
--     ;
-- END;
-- $$ LANGUAGE plpgsql STABLE STRICT;


-- CREATE TRIGGER register_vlob_atom_in_beacon_tg
--     AFTER INSERT ON vlob_atoms
--     FOR EACH ROW
--     EXECUTE PROCEDURE register_vlob_atom_in_beacon(vlob_atom);


-------------------------------------------------------
--  Blockstore
-------------------------------------------------------


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
