-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key,
    user_profile_outsider_allowed, active_users_limit, is_expired,
    _expired_on, _bootstrapped_on, _created_on
) VALUES (
    8000,
    'Org8000',
    '123456',
    E'\\x1234567890abcdef',
    TRUE,
    NULL,
    TRUE,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key,
    user_profile_outsider_allowed, active_users_limit, is_expired,
    _expired_on, _bootstrapped_on, _created_on
) VALUES (
    8001,
    'Org8001',
    '123456',
    E'\\x1234567890abcdef',
    TRUE,
    NULL,
    FALSE,
    NULL,
    NULL,
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key,
    user_profile_outsider_allowed, active_users_limit, is_expired,
    _expired_on, _bootstrapped_on, _created_on
) VALUES (
    8002,
    'Org8002',
    '123456',
    E'\\x1234567890abcdef',
    TRUE,
    NULL,
    FALSE,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key,
    user_profile_outsider_allowed, active_users_limit, is_expired,
    _expired_on, _bootstrapped_on, _created_on
) VALUES (
    8003,
    'Org8003',
    '123456',
    E'\\x1234567890abcdef',
    TRUE,
    NULL,
    TRUE,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    '2021-07-29 10:13:41.699846+00'
);
