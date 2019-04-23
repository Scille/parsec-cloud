-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
-------------------------------------------------------


CREATE TABLE vlob_group (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    vlob_group_id UUID NOT NULL,

    UNIQUE(organization, vlob_group_id)
);


CREATE TYPE vlob_group_role AS ENUM ('OWNER', 'MANAGER', 'CONTRIBUTOR', 'READER');


CREATE TABLE vlob_group_user_role (
    _id SERIAL PRIMARY KEY,
    vlob_group INTEGER REFERENCES vlob_group (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    role vlob_group_role NOT NULL,

    UNIQUE(vlob_group, user_)
);


CREATE TABLE vlob (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    vlob_group INTEGER REFERENCES vlob_group (_id),
    vlob_id UUID NOT NULL,

    UNIQUE(organization, vlob_id)
);


CREATE TABLE vlob_atom (
    _id SERIAL PRIMARY KEY,
    vlob INTEGER REFERENCES vlob (_id) NOT NULL,
    version INTEGER NOT NULL,
    blob BYTEA NOT NULL,
    author INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not deleted
    deleted_on TIMESTAMPTZ,

    UNIQUE(vlob, version)
);


CREATE TABLE vlob_group_update (
    _id SERIAL PRIMARY KEY,
    vlob_group INTEGER REFERENCES vlob_group (_id) NOT NULL,
    index INTEGER NOT NULL,
    vlob_atom INTEGER REFERENCES vlob_atom (_id) NOT NULL,

    UNIQUE(vlob_group, index)
);


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


CREATE FUNCTION user_has_vlob_group_admin_right(userinternalid INTEGER, vgroupinternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM vlob_group_user_role
        WHERE
            vlob_group = vgroupinternalid
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
        FROM vlob_group_user_role
        WHERE
            vlob_group = vgroupinternalid
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
        FROM vlob_group_user_role
        WHERE
            vlob_group = vgroupinternalid
            AND user_ = userinternalid
            AND role != 'READER'
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_group_internal_id(orgid VARCHAR, vgroupid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM vlob_group
        WHERE
            organization = get_organization_internal_id(orgid)
            AND vlob_group_id = vgroupid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_group_id(vgroupinternalid INTEGER) RETURNS UUID AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            vlob_group_id
        FROM vlob_group
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
