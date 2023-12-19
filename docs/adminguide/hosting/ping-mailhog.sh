#!/usr/bin/env bash

set -a
source parsec-email.env

curl \
    --url "smtp://127.0.0.1:$PARSEC_EMAIL_PORT" \
    --user "$PARSEC_EMAIL_HOST_USER@localhost:$PARSEC_EMAIL_HOST_PASSWORD" \
    --mail-from $PARSEC_EMAIL_SENDER \
    --mail-rcpt rcpt@test.com \
    --upload-file <(date --rfc-3339=seconds)
