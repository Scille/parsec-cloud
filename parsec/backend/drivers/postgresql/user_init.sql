-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


-------------------------------------------------------
--  Tables
-------------------------------------------------------


CREATE TABLE user_ (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_id VARCHAR(32) NOT NULL,
    is_admin BOOLEAN NOT NULL,
    certified_user BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    user_certifier INTEGER,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, user_id)
);


CREATE TABLE device (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,
    device_id VARCHAR(65) NOT NULL,
    certified_device BYTEA NOT NULL,
    -- NULL if certifier is the Root Verify Key
    device_certifier INTEGER REFERENCES device (_id),
    created_on TIMESTAMPTZ NOT NULL,
    -- NULL if not yet revocated
    revocated_on TIMESTAMPTZ,
    -- NULL if not yet revocated
    certified_revocation BYTEA,
    -- NULL if certifier is the Root Verify Key
    revocation_certifier INTEGER REFERENCES device (_id),

    UNIQUE(organization, device_id),
    UNIQUE(user_, device_id)
);


ALTER TABLE user_
ADD CONSTRAINT FK_user_device FOREIGN KEY (user_certifier) REFERENCES device (_id);


CREATE TABLE user_invitation (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_id VARCHAR(32) NOT NULL,
    creator INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, user_id)
);


CREATE TABLE device_invitation (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    device_id VARCHAR(65) NOT NULL,
    creator INTEGER REFERENCES device (_id) NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,

    UNIQUE(organization, device_id)
);


-------------------------------------------------------
--  Procedures
-------------------------------------------------------


CREATE FUNCTION get_user_internal_id(orgid VARCHAR, userid VARCHAR) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT
            _id
        FROM user_
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
        FROM user_
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
        FROM device
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
        FROM device
        WHERE
            _id = deviceinternalid
    );
END;
$$ LANGUAGE plpgsql STABLE STRICT;
