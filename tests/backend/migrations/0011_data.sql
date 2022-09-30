-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


INSERT INTO organization(
    _id, organization_id, bootstrap_token, root_verify_key,
    user_profile_outsider_allowed, active_users_limit, is_expired,
    _expired_on, _bootstrapped_on, _created_on,
    sequester_authority_certificate, sequester_authority_verify_key_der
) VALUES (
    11000,
    'Org11000',
    '123456',
    E'\\x1234567890abcdef',
    TRUE,
    NULL,
    TRUE,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef'
);


INSERT INTO sequester_service(
    _id, service_id, organization, service_certificate, service_label, created_on
) VALUES (
    11000,
    '6ecd8c99-4036-403d-bf84-cf8400f67836',
    11000,
    E'\\x1234567890abcdef',
    'service',
    '2021-07-29 10:13:41.699846+00'
);


INSERT INTO sequester_service(
    _id, service_id, organization, service_certificate, service_label, created_on, disabled_on
) VALUES (
    11001,
    '6ecd8c99-4036-403d-bf84-cf8400f67831',
    11000,
    E'\\x1234567890abcdef',
    'disabled_service',
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00'
);

INSERT INTO sequester_service_vlob_atom(
    _id, vlob_atom, service, blob
) VALUES (
    11000,
    502,
    11000,
    E'\\x1234567890abcdef'
);
