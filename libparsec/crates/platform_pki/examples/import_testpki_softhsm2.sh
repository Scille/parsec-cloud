#!/usr/bin/env bash
set -eu

SOFTHSM2_SLOT=${SOFTHSM2_SLOT:?Need to defined a slot to use}
TEST_PKCS11_MODULE=${TEST_PKCS11_MODULE:?Need a path to a dynlib providing a pkcs11 interface}
SLOT_USER_PIN=${SLOT_USER_PIN:=1234}

IMPORT_PATH=${1:?"Need import path"}

PKCS11_TOOL_SHARED_ARGS=("--module=$TEST_PKCS11_MODULE" "--login" "--pin=$SLOT_USER_PIN" "--slot=$SOFTHSM2_SLOT")

function import_object {
    pkcs11-tool "${PKCS11_TOOL_SHARED_ARGS[@]}" --id="$1" --write-object="$2" --type "$3"
}

function import_certificate {
    import_object "$1" "$2" cert
}

function get_id_from_cert {
    openssl sha256 -hex -r "$1" | cut -c 10
}

for cert in "$IMPORT_PATH"/Root/*.crt; do
    id=$(get_id_from_cert "$cert")
    import_certificate "$id" "$cert"
done

for cert in "$IMPORT_PATH"/Intermediate/*.crt; do
    id=$(get_id_from_cert "$cert")
    import_certificate "$id" "$cert"
done

for cert in "$IMPORT_PATH"/Cert/*.crt; do
    id=$(get_id_from_cert "$cert")
    import_certificate "$id" "$cert"
    key_file=${cert/%.crt/.key}
    import_object "$id" "$key_file" privkey
done
