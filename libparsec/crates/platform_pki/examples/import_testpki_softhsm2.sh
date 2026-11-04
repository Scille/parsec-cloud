#!/usr/bin/env bash
set -eu

# cspell: words libsofthsm

# SOFTHSM2_SLOT & SOFTHSM2_CONF are environ variables needed by `libsofthsm2.so`
SOFTHSM2_CONF=${SOFTHSM2_CONF:?Need to set SOFTHSM2_CONF to the softhsm2 configuration file path}
SOFTHSM2_SLOT=${SOFTHSM2_SLOT:?Need to defined a slot to use for the softhsm2 token}
SLOT_USER_PIN=${SLOT_USER_PIN:=1234}
TEST_PKCS11_MODULE=${TEST_PKCS11_MODULE:-$(find /usr/lib /usr/local/lib -name "libsofthsm2.so" 2>/dev/null | head -1)}

if [ -z "$TEST_PKCS11_MODULE" ]; then
    echo "error: libsofthsm2.so not found (do `sudo apt install softhsm2` ?)" >&2
    exit 1
fi
if ! command -v pkcs11-tool &>/dev/null; then
    echo "error: pkcs11-tool not found (do `sudo apt install opensc` ?)" >&2
    exit 1
fi

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
IMPORT_PATH="$SCRIPT_DIR/../test-pki"

PKCS11_TOOL_SHARED_ARGS=("--module=$TEST_PKCS11_MODULE" "--login" "--pin=$SLOT_USER_PIN" "--slot=$SOFTHSM2_SLOT")

function import_object {
    # Run in a subshell with `set -x` to display the command before running it
    ( set -x; pkcs11-tool "${PKCS11_TOOL_SHARED_ARGS[@]}" --id="$1" --write-object="$2" --type "$3" )
}

function import_certificate {
    import_object "$1" "$2" cert
}

function get_id_from_cert {
    openssl sha256 -hex -r "$1" | cut --characters -10
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

for crl in "$IMPORT_PATH"/CRL/*.crl; do
    id=$(get_id_from_cert "$crl")
    import_object "$id" "$crl" data
done
