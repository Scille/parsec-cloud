-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

-------------------------------------------------------
--  Migration
-------------------------------------------------------


CREATE TABLE shamir_recovery_setup (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,
    user_ INTEGER REFERENCES user_ (_id) NOT NULL,

    brief_certificate BYTEA NOT NULL,
    reveal_token UUID NOT NULL,
    threshold INTEGER NOT NULL,
    shares INTEGER NOT NULL,
    ciphered_data BYTEA,

    UNIQUE(organization, reveal_token)
);


CREATE TABLE shamir_recovery_share (
    _id SERIAL PRIMARY KEY,
    organization INTEGER REFERENCES organization (_id) NOT NULL,

    shamir_recovery INTEGER REFERENCES shamir_recovery_setup (_id) NOT NULL,
    recipient INTEGER REFERENCES user_ (_id) NOT NULL,

    share_certificate BYTEA NOT NULL,
    shares INTEGER NOT NULL,

    UNIQUE(organization, shamir_recovery, recipient)
);

CREATE TABLE shamir_recovery_conduit (
    _id SERIAL PRIMARY KEY,
    invitation INTEGER REFERENCES invitation (_id) NOT NULL,
    greeter INTEGER REFERENCES user_ (_id) NOT NULL,

    conduit_state invitation_conduit_state NOT NULL DEFAULT '1_WAIT_PEERS',
    conduit_greeter_payload BYTEA,
    conduit_claimer_payload BYTEA,

    UNIQUE(invitation, greeter)
);



ALTER TABLE user_ ADD shamir_recovery INTEGER REFERENCES shamir_recovery_setup (_id);
ALTER TABLE invitation ADD shamir_recovery INTEGER REFERENCES shamir_recovery_setup (_id);
ALTER TYPE invitation_type ADD VALUE 'SHAMIR_RECOVERY';
