-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
-------------------------------------------------------

CREATE TYPE maintenance_type AS ENUM ('REENCRYPTION', 'GARBAGE_COLLECTION');


CREATE TABLE realm (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    realm_id UUID NOT NULL,
    encryption_revision int NOT NULL,
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
    role realm_role NOT NULL,

    UNIQUE(realm, user_)
);


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


CREATE FUNCTION user_has_realm_maintenance_access(userinternalid INTEGER, realminternalid INTEGER) RETURNS BOOLEAN AS $$
DECLARE
BEGIN
    RETURN EXISTS (
        SELECT
            true
        FROM realm_user_role
        WHERE
            realm = realminternalid
            AND user_ = userinternalid
            AND role = 'OWNER'
        LIMIT 1
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION user_has_realm_read_access(userinternalid INTEGER, realminternalid INTEGER) RETURNS BOOLEAN AS $$
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


CREATE FUNCTION user_has_realm_write_access(userinternalid INTEGER, realminternalid INTEGER) RETURNS BOOLEAN AS $$
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


CREATE FUNCTION get_realm_internal_id(orgid VARCHAR, realmid UUID) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM realm
        WHERE
            organization = get_organization_internal_id(orgid)
            AND realm_id = realmid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;


CREATE FUNCTION get_realm_id(realminternalid INTEGER) RETURNS UUID AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            realm_id
        FROM realm
        WHERE _id = realminternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;
