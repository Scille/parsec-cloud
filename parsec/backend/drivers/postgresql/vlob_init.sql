-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
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


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


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
