-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key, expiration_date, user_profile_outsider_allowed
) VALUES (
    6000,
    'Org6000',
    '123456',
    E'\\x1234567890abcdef',
    '2021-07-29 10:13:41.699846+00',
    TRUE
);

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key, expiration_date, user_profile_outsider_allowed
) VALUES (
    6001,
    'Org6001',
    '123456',
    E'\\x1234567890abcdef',
    '2021-07-29 10:13:41.699846+00',
    FALSE
);
