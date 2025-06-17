#!/usr/bin/env bash
set -euo pipefail

ENV_FILE=parsec-admin-token.env
if [ ! -f $ENV_FILE ]; then
    TOKEN=$(openssl rand -hex 32)
    echo "PARSEC_ADMINISTRATION_TOKEN=$TOKEN" > $ENV_FILE
    echo "Parsec administration token generated in: $ENV_FILE"
else
    echo "Parsec administration token already exists in: $ENV_FILE"
fi
