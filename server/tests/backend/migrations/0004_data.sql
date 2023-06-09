-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4000,
    10,
    '3458c7ec-626b-41da-b9eb-cf8164baa487',
    'USER',
    20,
    'adam@example.com',
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '1_WAIT_PEERS',
    NULL,
    NULL
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4001,
    10,
    'e0f85b59-78f8-4188-acfc-2e2a51360f4c',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    'FINISHED',
    '1_WAIT_PEERS',
    NULL,
    NULL
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4002,
    10,
    '1027c25b-912d-4a5c-9407-294df9f1b51c',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    'CANCELLED',
    '1_WAIT_PEERS',
    NULL,
    NULL
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4003,
    10,
    'dd0c73c5-16fe-4eb4-8c45-f8a77cb90288',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    '2021-07-29 10:13:41.699846+00',
    'ROTTEN',
    '1_WAIT_PEERS',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef'
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4004,
    10,
    'd28a9224-a791-40cb-a44d-42d1cab3e4c0',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '1_WAIT_PEERS',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef'
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4005,
    10,
    '2decca1f-edfb-4f45-b58a-ff499d0bc5fe',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '2_1_CLAIMER_HASHED_NONCE',
    NULL,
    E'\\x1234567890abcdef'
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4006,
    10,
    'a2b9aa15-7e8b-4144-bfa9-01a9ddde08c6',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '2_2_GREETER_NONCE',
    E'\\x1234567890abcdef',
    NULL
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4007,
    10,
    'ebd5b84e-59f7-4b41-84bd-aa07af174811',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '2_3_CLAIMER_NONCE',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef'
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4008,
    10,
    'c6c503ab-a724-4bc0-8f4a-ff80e40e16fa',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '3_1_CLAIMER_TRUST',
    E'\\x1234567890abcdef',
    NULL
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4009,
    10,
    '94fe8ad1-ea10-40ab-9b72-f3e13a615d40',
    'DEVICE',
    20,
    NULL,
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '3_2_GREETER_TRUST',
    NULL,
    E'\\x1234567890abcdef'
);

INSERT INTO invitation(
    _id, organization, token, type, greeter, claimer_email, created_on, deleted_on,
    deleted_reason, conduit_state, conduit_greeter_payload, conduit_claimer_payload
) VALUES (
    4010,
    10,
    '788f0f67-cff9-406d-a8ea-bb59f71fdc15',
    'USER',
    20,
    'adam@example.com',
    '2021-07-29 10:13:41.699846+00',
    NULL,
    NULL,
    '4_COMMUNICATE',
    E'\\x1234567890abcdef',
    E'\\x1234567890abcdef'
);
