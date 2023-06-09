-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

-- Updating a role now updates the realm_user_change table
-- Here, alice@pc1 (user 20 device 30) grants contributor access to bob (user 21) to the realm 200

INSERT INTO realm_user_role(_id, realm, user_, role, certificate, certified_by, certified_on) VALUES (
    307,
    200,
    21,
    'CONTRIBUTOR',
    E'\\x1234567890abcdef',
    30,
    '2021-10-29 11:58:08.841265+02'
);

INSERT INTO realm_user_change(_id, realm, user_, last_role_change, last_vlob_update) VALUES (
    2,
    200,
    20,
    '2021-10-29 11:58:08.841265+02',
    NULL
);

-- Writing a vlob now updates the realm_user_change table
-- Here, bob@pc1 (user 21 device 32) writes a vlob to the realm 200

INSERT INTO vlob_atom(_id, organization, vlob_encryption_revision, vlob_id, version, blob, size, author, created_on, deleted_on) VALUES (
    502,
    10,
    400,
    '3458c7ec-626b-41da-b9eb-cf8164baa487',
    2,
    E'\\x1234567890abcdef',
    24,
    32,
    '2021-10-29 11:30:16.791954+02',
    NULL
);

INSERT INTO realm_vlob_update(_id, realm, index, vlob_atom) VALUES (
    602,
    200,
    3,
    502
);

INSERT INTO realm_user_change(_id, realm, user_, last_role_change, last_vlob_update) VALUES (
    1,
    200,
    21,
    NULL,
    '2021-10-29 11:30:16.791954+02'
);
