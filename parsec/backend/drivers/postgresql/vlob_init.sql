-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
-------------------------------------------------------


CREATE TABLE realm (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm_id UUID NOT NULL,
    encryption_revision int NOT NULL,

    UNIQUE(organization, realm_id)
);


CREATE TYPE realm_role AS ENUM ('OWNER', 'MANAGER', 'CONTRIBUTOR', 'READER');


CREATE TABLE realm_user_role (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    role realm_role NOT NULL,

    UNIQUE(realm, user_)
);


CREATE TABLE vlob (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm INTEGER REFERENCES realm (_id),
    vlob_id UUID NOT NULL,

    UNIQUE(organization, vlob_id)
);


CREATE TABLE vlob_atom (
    _id SERIAL PRIMARY KEY,
    # TODO: Add organization and/or realm here ?
    encryption_revision INTEGER NOT NULL,
    vlob INTEGER REFERENCES vlob (_id) NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ,

    UNIQUE(vlob, version)
);


CREATE TABLE realm_update (
    _id SERIAL PRIMARY KEY,
    realm INTEGER REFERENCES realm (_id) NOT NULL,
    index INTEGER NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,

    UNIQUE(realm, index)
);


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


CREATE FUNCTION user_has_realm_admin_right(userinternalid INTEGER, vgroupinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = vgroupinternalid
            AND user_ = userinternalid
            AND admin_right = TRUE
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_can_read_vlob(userinternalid INTEGER, vgroupinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = vgroupinternalid
            AND user_ = userinternalid
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_can_write_vlob(userinternalid INTEGER, vgroupinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = vgroupinternalid
            AND user_ = userinternalid
            AND role != 'READER'
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_realm_internal_id(orgid VARCHAR, vgroupid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM realm
        WHERE
            organization = get_organization_internal_id(orgid)
            AND realm_id = vgroupid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_realm_id(vgroupinternalid INTEGER) RETURNS UUID AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            realm_id
        FROM realm
        WHERE _id = vgroupinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_internal_id(orgid VARCHAR, vlobid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM vlob
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
        FROM vlob
        WHERE _id = vlobinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;
