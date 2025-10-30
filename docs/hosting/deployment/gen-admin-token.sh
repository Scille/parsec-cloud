# shellcheck disable=SC2148
set -euo pipefail

ENV_FILE=parsec-admin-token.env
if [ ! -f $ENV_FILE ]; then
    PARSEC_ADMINISTRATION_TOKEN=$(openssl rand -hex 32)
    echo "PARSEC_ADMINISTRATION_TOKEN=$PARSEC_ADMINISTRATION_TOKEN" > $ENV_FILE

    PARSEC_FAKE_ACCOUNT_PASSWORD_ALGORITHM_SEED=$(openssl rand -hex 32)
    echo "PARSEC_FAKE_ACCOUNT_PASSWORD_ALGORITHM_SEED=$PARSEC_FAKE_ACCOUNT_PASSWORD_ALGORITHM_SEED" >> $ENV_FILE

    echo "Parsec administration token generated in: $ENV_FILE"
else
    echo "Parsec administration token already exists in: $ENV_FILE"
fi
