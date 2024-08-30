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

CREATE TABLE greeting_session (
    _id SERIAL PRIMARY KEY,
    invitation INTEGER REFERENCES invitation (_id) NOT NULL,
    greeter INTEGER REFERENCES user_ (_id) NOT NULL,

    UNIQUE (invitation, greeter)
);

CREATE TABLE greeting_attempt (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    greeting_attempt_id UUID NOT NULL,
    greeting_session INTEGER REFERENCES greeting_session (_id) NOT NULL,

    claimer_joined TIMESTAMPTZ DEFAULT NULL,
    greeter_joined TIMESTAMPTZ DEFAULT NULL,

    cancelled_reason CANCELLED_GREETING_ATTEMPT_REASON DEFAULT NULL,
    cancelled_on TIMESTAMPTZ DEFAULT NULL,
    cancelled_by GREETER_OR_CLAIMER DEFAULT NULL,

    UNIQUE (organization, greeting_attempt_id)
);

-- Makes sure that there is only one active greeting attempt per session
CREATE UNIQUE INDEX unique_active_attempt ON greeting_attempt (greeting_session)
WHERE cancelled_on IS NULL;

CREATE TABLE greeting_step (
    _id SERIAL PRIMARY KEY,
    greeting_attempt INTEGER REFERENCES greeting_attempt (_id) NOT NULL,
    step INTEGER NOT NULL,
    greeter_data BYTEA,
    claimer_data BYTEA,

    UNIQUE (greeting_attempt, step)
);
