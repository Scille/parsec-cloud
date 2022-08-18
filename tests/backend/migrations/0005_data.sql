-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

INSERT INTO user_(
    _id, organization, user_id, profile, user_certificate, redacted_user_certificate,
    user_certifier, created_on, revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    5000,
    10,
    'riri',
    'ADMIN',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    NULL
);

INSERT INTO user_(
    _id, organization, user_id, profile, user_certificate, redacted_user_certificate,
    user_certifier, created_on, revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    5001,
    10,
    'fifi',
    'STANDARD',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    NULL
);

INSERT INTO user_(
    _id, organization, user_id, profile, user_certificate, redacted_user_certificate,
    user_certifier, created_on, revoked_on, revoked_user_certificate, revoked_user_certifier
) VALUES (
    5002,
    10,
    'loulou',
    'OUTSIDER',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    NULL
);

INSERT INTO device(
    _id, organization, user_, device_id, device_label, device_certificate,
    redacted_device_certificate, device_certifier, created_on
) VALUES (
    5010,
    10,
    5000,
    'riri@pc1',
    'PC 1',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef',
    31,
    '2021-07-29 10:13:41.699846+00'
);
