-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
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


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


CREATE FUNCTION user_can_read_vlob(userinternalid INTEGER, realminternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = realminternalid
            AND user_ = userinternalid
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_can_write_vlob(userinternalid INTEGER, realminternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = realminternalid
            AND user_ = userinternalid
            AND role != 'READER'
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_encryption_revision_internal_id(orgid VARCHAR, realmid UUID, revision INTEGER) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM vlob_encryption_revision
        WHERE
            realm = get_realm_internal_id(orgid, realmid)
            AND encryption_revision = revision
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_atom_internal_id(orgid VARCHAR, vlobid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM vlob_atom
        WHERE
            organization = get_organization_internal_id(orgid)
            AND vlob_id = vlobid
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_vlob_id(vlobatominternalid INTEGER) RETURNS UUID AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            vlob_id
        FROM vlob_atom
        WHERE _id = vlobatominternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;
