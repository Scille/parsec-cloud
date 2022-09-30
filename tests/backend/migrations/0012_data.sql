-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


INSERT INTO sequester_service(
    _id, service_id, organization, service_certificate, service_label, created_on, service_type
) VALUES (
    12000,
    '6ecd8c99-4036-403d-bf84-cf8400f67864',
    11000,
    E'\\x1234567890abcdef',
    'service',
    '2021-07-29 10:13:41.699846+00',
    'STORAGE'
);


INSERT INTO sequester_service(
    _id, service_id, organization, service_certificate, service_label, created_on, disabled_on, webhook_url, service_type
) VALUES (
    12001,
    '6ecd8c99-4036-403d-bf84-cf8400f67814',
    11000,
    E'\\x1234567890abcdef',
    'disabled_service',
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    'http://somewhere.lost',
    'WEBHOOK'
);

INSERT INTO sequester_service_vlob_atom(
    _id, vlob_atom, service, blob
) VALUES (
    12000,
    502,
    12001,
    E'\\x1234567890abcdef'
);
