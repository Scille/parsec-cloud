#!/usr/bin/env bash

ENV_FILE=parsec-admin-token.env
if [ ! -f $ENV_FILE ]; then
    TOKEN=$(openssl rand 63 | base64 --wrap=86)
    echo "PARSEC_ADMINISTRATION_TOKEN=$TOKEN" > ENV_FILE
    echo "Parsec administration token generated in: $ENV_FILE"
else
    echo "Parsec administration token already exists in: $ENV_FILE"
fi
