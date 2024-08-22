-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


CREATE TYPE cancelled_greeting_attempt_reason AS ENUM (
    'MANUALLY_CANCELLED',
    'INVALID_NONCE_HASH',
    'INVALID_SAS_CODE',
    'UNDECIPHERABLE_PAYLOAD',
    'UNDESERIALIZABLE_PAYLOAD',
    'INCONSISTENT_PAYLOAD',
    'AUTOMATICALLY_CANCELLED'
);

CREATE TYPE greeter_or_claimer AS ENUM (
    'GREETER',
    'CLAIMER'
);

CREATE TABLE greeting_sessions (
    _id SERIAL PRIMARY KEY,
    invitation INTEGER REFERENCES invitation (_id) NOT NULL,
    greeter INTEGER REFERENCES user_ (_id) NOT NULL
);

CREATE TABLE greeting_attempts (
    _id SERIAL PRIMARY KEY,
    invitation INTEGER REFERENCES invitation (_id) NOT NULL,
    greeter INTEGER REFERENCES user_ (_id) NOT NULL,
    greeting_session INTEGER REFERENCES greeting_sessions (_id) NOT NULL,

    claimer_joined TIMESTAMPTZ DEFAULT NULL,
    greeter_joined TIMESTAMPTZ DEFAULT NULL,

    cancelled_reason CANCELLED_GREETING_ATTEMPT_REASON DEFAULT NULL,
    cancelled_on TIMESTAMPTZ DEFAULT NULL,
    cancelled_by GREETER_OR_CLAIMER DEFAULT NULL,
    greeter_steps BYTEA [],
    claimer_steps BYTEA []

);
