-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

INSERT INTO human(_id, organization, email, label) VALUES (
    3000,
    10,
    'James T. Kirk',
    'kirk@starfleet.com'
);

INSERT INTO user_(
    _id, organization, user_id, human, is_admin, user_certificate, user_certifier,
    created_on, revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    3001,
    10,
    'kirk',
    3000,
    TRUE,
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    NULL
);

INSERT INTO device(
    _id, organization, user_, device_id, device_certificate, device_certifier, created_on
) VALUES (
    3002,
    10,
    3001,
    'kirk@enterprise',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00'
);
