-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


CREATE TABLE organization (
    _id SERIAL PRIMARY KEY,
    organization_id VARCHAR(32) UNIQUE NOT NULL,
    bootstrap_token TEXT NOT NULL,
    root_verify_key BYTEA
);


CREATE FUNCTION get_organization_internal_id(VARCHAR) RETURNS INTEGER AS $$
DECLARE
BEGIN
    RETURN (
        SELECT _id from organization WHERE organization_id = $1
    );
END;
$$ LANGUAGE plpgsql STABLE;


CREATE FUNCTION get_organization_id(INTEGER) RETURNS VARCHAR AS $$
DECLARE
BEGIN
    RETURN (
        SELECT organization_id from organization WHERE _id = $1
    );
END;
$$ LANGUAGE plpgsql STABLE;
