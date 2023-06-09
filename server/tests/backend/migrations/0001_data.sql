-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

-- Create organizations

INSERT INTO organization(_id, organization_id, bootstrap_token, root_verify_key, expiration_date) VALUES (
    10,
    'CoolOrg',
    '123456',
    E'\\x1234567890abcdef',
    '2021-07-29 10:13:41.699846+00'
);
INSERT INTO organization(_id, organization_id, bootstrap_token, root_verify_key, expiration_date) VALUES (
    11,
    'NotBootstrappedOrganization',
    'abcdef',
    NULL,
    NULL
);

-- Create Alice

INSERT INTO user_(
    _id, organization, user_id, is_admin, user_certificate, user_certifier, created_on,
    revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    20,
    10,
    'alice',
    TRUE,
    E'\\x1234567890abcdef',
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    NULL
);
INSERT INTO device(_id, organization, user_, device_id, device_certificate, device_certifier, created_on) VALUES (
    30,
    10,
    20,
    'alice@pc1',
    E'\\x1234567890abcdef',
    NULL,
    '2021-07-29 10:13:41.699846+00'
);
INSERT INTO device_invitation(_id, organization, device_id, creator, created_on) VALUES (
    50,
    10,
    'alice@pc2',
    30,
    '2021-07-29 10:13:41.699846+00'
);
INSERT INTO device(_id, organization, user_, device_id, device_certificate, device_certifier, created_on) VALUES (
    31,
    10,
    20,
    'alice@pc2',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

-- Create Bob

INSERT INTO user_invitation(_id, organization, user_id, creator, created_on) VALUES (
    40,
    10,
    'bob',
    31,
    '2021-07-29 10:13:41.699846+00'
);
INSERT INTO user_(
    _id, organization, user_id, is_admin, user_certificate, user_certifier, created_on,
    revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    21,
    10,
    'bob',
    FALSE,
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    E'\\x1234567890abcdef',
    30
);
INSERT INTO device(_id, organization, user_, device_id, device_certificate, device_certifier, created_on) VALUES (
    32,
    10,
    21,
    'bob@pc1',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00'
);

-- Messages

INSERT INTO message(_id, organization, recipient, timestamp, index, sender, body) VALUES (
    100,
    10,
    21,
    '2021-07-29 10:13:41.699846+00',
    1,
    31,
    E'\\x1234567890abcdef'
);

--  Realms

INSERT INTO realm(_id, organization, realm_id, encryption_revision, maintenance_started_by, maintenance_started_on, maintenance_type) VALUES (
    200,
    10,
    'd8602e9b-85ff-4f1e-bdc2-786571470b3d',
    1,
    30,
    '2021-07-29 10:13:41.699846+00',
    'REENCRYPTION'
);

INSERT INTO realm(_id, organization, realm_id, encryption_revision, maintenance_started_by, maintenance_started_on, maintenance_type) VALUES (
    201,
    10,
    '48a8d192-b221-4e74-8acb-7b57c310894e',
    1,
    31,
    '2021-07-29 10:13:41.699846+00',
    'GARBAGE_COLLECTION'
);

INSERT INTO realm(_id, organization, realm_id, encryption_revision, maintenance_started_by, maintenance_started_on, maintenance_type) VALUES (
    202,
    10,
    'ab1f22f3-1e5c-495c-a53d-2767e4775561',
    1,
    NULL,
    NULL,
    NULL
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    300,
    200,
    20,
    'OWNER',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    301,
    201,
    20,
    'OWNER',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    302,
    202,
    20,
    'OWNER',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    303,
    200,
    21,
    'MANAGER',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    304,
    200,
    21,
    'CONTRIBUTOR',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    305,
    200,
    21,
    'READER',
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    306,
    200,
    21,
    NULL,
    E'\\x1234567890abcdef',
    30,
    '2021-07-29 10:13:41.699846+00'
);

--  Vlob

INSERT INTO vlob_encryption_revision(_id, realm, encryption_revision) VALUES (
    400,
    200,
    1
);

INSERT INTO vlob_atom(_id, organization, vlob_encryption_revision, vlob_id, version, blob, size, author, created_on, deleted_on) VALUES (
    500,
    10,
    400,
    '3458c7ec-626b-41da-b9eb-cf8164baa487',
    1,
    E'\\x1234567890abcdef',
    24,
    31,
    '2021-07-29 10:13:41.699846+00',
    NULL
);

INSERT INTO vlob_atom(_id, organization, vlob_encryption_revision, vlob_id, version, blob, size, author, created_on, deleted_on) VALUES (
    501,
    10,
    400,
    'e0f85b59-78f8-4188-acfc-2e2a51360f4c',
    1,
    E'\\x1234567890abcdef',
    24,
    31,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO realm_vlob_update(_id, realm, index, vlob_atom) VALUES (
    600,
    200,
    1,
    500
);

INSERT INTO realm_vlob_update(_id, realm, index, vlob_atom) VALUES (
    601,
    200,
    2,
    501
);

--  Block

INSERT INTO block(_id, organization, block_id, realm, author, size, created_on, deleted_on) VALUES (
    700,
    10,
    '11d2bdea-5b1d-41ad-9da0-7570a3d666d6',
    200,
    31,
    24,
    '2021-07-29 10:13:41.699846+00',
    NULL
);

INSERT INTO block(_id, organization, block_id, realm, author, size, created_on, deleted_on) VALUES (
    701,
    10,
    'ce461256-b21b-41d1-a589-5a41fb0821cf',
    200,
    31,
    24,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO block_data(_id, organization_id, block_id, data) VALUES (
    800,
    'CoolOrg',
    '11d2bdea-5b1d-41ad-9da0-7570a3d666d6',
    E'\\x1234567890abcdef'
);

INSERT INTO block_data(_id, organization_id, block_id, data) VALUES (
    801,
    'CoolOrg',
    'ce461256-b21b-41d1-a589-5a41fb0821cf',
    E'\\x1234567890abcdef'
);
