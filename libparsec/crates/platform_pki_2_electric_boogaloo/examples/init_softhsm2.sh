#!/usr/bin/env bash

# This script need to have a softhsm2 to be initialized
# Bootstrap:
#
# 1. Create a directory that will contain the tokens created by `softhsm2` (e.g. `/tmp/softhsm/tokens`), set the env var `SOFTHSM2_TOKENPATH` to that path
# 2. Create a file with the following content, and set `SOFTHSM2_CONF` to its path:
#    ```ini
#    directories.tokendir = /tmp/softhsm/tokens
#    ```
#    > Replace `/tmp/softhsm/tokens` with the value of `SOFTHSM2_TOKENPATH`
# 3. Initialize your first token:
#    ```shell
#    export SLOT_USER_PIN=1234
#    softhsm2-util --init-token --slot 0 --label "My first token" --pin $SLOT_USER_PIN --so-pin 123456 | tee /dev/stderr | rev | cut -d ' ' -f 1 | rev > $SOFTHSM2_TOKENPATH/work-slot
#    export SOFTHSM2_SLOT=$(cat $SOFTHSM2_TOKENPATH/work-slot)
#    ```
# To reset the HSM, simply remove `$SOFTHSM2_TOKENPATH` folder and redo the above procedure

set -eu

SOFTHSM2_SLOT=${SOFTHSM2_SLOT:?Need to defined a slot to use}
TEST_PKCS11_MODULE=${TEST_PKCS11_MODULE:?Need a path to a dynlib providing a pkcs11 interface}
SLOT_USER_PIN=${SLOT_USER_PIN:=1234}
WORKDIR=${SOFTHSM2_TOKENPATH:=/tmp/parsec-dev-softhsm}

if ! [ -f "$WORKDIR/openssl.conf" ]; then
    cat >"$WORKDIR/openssl.conf" <<EOF
keyUsage = digitalSignature
EOF
fi

echo Creating Test CA
openssl req -x509 -nodes -newkey rsa:2048 -keyout "$WORKDIR/ca.key" -out "$WORKDIR/ca.pem" -sha256 -days 365 -subj '/CN=Parsec Test CA' -addext 'keyUsage = digitalSignature'

declare -a users
users+=(alice)
users+=(bob)

for user in "${users[@]}"; do
    echo "Creating $user Certificate Signature Request"
    email="${user}@example.com"
    openssl req -new -nodes -newkey rsa:3096 \
        -keyout "$WORKDIR/$user.key" -out "$WORKDIR/$user.csr" \
        -subj "/CN=${user} Parsec Test/emailAddress=${email}" \
        -addext "subjectAltName=email:san+${email}"

    echo "Signing $user CSR"
    openssl x509 -req -days 365 \
        -in "$WORKDIR/$user.csr" \
        -CA "$WORKDIR/ca.pem" -CAkey "$WORKDIR/ca.key" -CAcreateserial \
        -out "$WORKDIR/$user.crt" -extfile "$WORKDIR/openssl.conf" \
        -copy_extensions copyall # Copy every extensions, notably the SAN email attr

    echo "Export $user RSA privkey into DER format"
    openssl rsa -in "$WORKDIR/$user.key" -outform DER -out "$WORKDIR/$user-key.der"
    echo "Format $user certificate into DER format"
    openssl x509 -outform DER -in "$WORKDIR/$user.crt" -out "$WORKDIR/$user-crt.der"
    echo "Export $user pubkey"
    openssl rsa -in "$WORKDIR/$user.key" -pubout -out "$WORKDIR/$user.pub"

    id=$(echo -n "$user" | openssl sha256 -hex -r | cut -c -10)
    PKCS11_TOOL_SHARED_ARGS=("--module=$TEST_PKCS11_MODULE" "--login" "--pin=$SLOT_USER_PIN" "--slot=$SOFTHSM2_SLOT" "--id=${id}")
    echo "Importing $user certificate into pkcs11 module"
    pkcs11-tool "${PKCS11_TOOL_SHARED_ARGS[@]}" --write-object "$WORKDIR/$user-crt.der" --type cert --label "$user certificate"
    echo "Importing $user privkey into pkcs11 module"
    pkcs11-tool "${PKCS11_TOOL_SHARED_ARGS[@]}" --write-object "$WORKDIR/$user-key.der" --type privkey --label "$user private key"
    echo "Importing $user pubkey into pkcs11 module"
    pkcs11-tool "${PKCS11_TOOL_SHARED_ARGS[@]}" --write-object "$WORKDIR/$user.pub" --type pubkey --label "$user public key"
done
