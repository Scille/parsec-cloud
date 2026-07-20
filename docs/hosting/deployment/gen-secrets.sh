# shellcheck disable=SC2148
set -euo pipefail

SECRETS_FOLDER=secrets

mkdir -p $SECRETS_FOLDER

PARSEC_ADMIN_ENV_FILE=$SECRETS_FOLDER/parsec-admin-token.env
if [ ! -f $PARSEC_ADMIN_ENV_FILE ]; then
    (
        echo "PARSEC_ADMINISTRATION_TOKEN=$(openssl rand -hex 32)"
        echo "PARSEC_FAKE_ACCOUNT_PASSWORD_ALGORITHM_SEED=$(openssl rand -hex 32)"
    ) | tee $PARSEC_ADMIN_ENV_FILE
else
    cat $PARSEC_ADMIN_ENV_FILE
fi

RUSTFS_ENV_FILE=$SECRETS_FOLDER/rustfs.env
if [ ! -f $RUSTFS_ENV_FILE ]; then
    (
        echo "RUSTFS_ACCESS_KEY=$(openssl rand -hex 16)"
        echo "RUSTFS_SECRET_KEY=$(openssl rand -base64 32)"
    ) | tee $RUSTFS_ENV_FILE
else
    cat $RUSTFS_ENV_FILE
fi
