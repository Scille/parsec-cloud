# shellcheck disable=SC2148
# cspell:words cfssl gencert initca cfssljson tonumber
set -e

if ! command -v cfssl; then
    echo "Command 'cfssl' not found, you need to install it to use this script" >&2
    exit 1
fi

mkdir -p tls

if [ ! -f tls/ca-key.pem ] || [ ca-csr.json -nt tls/ca-key.pem ]; then
    cfssl gencert -initca ca-csr.json | cfssljson -bare tls/ca
fi

# Number of days the certificates are valid according to the config file.
# NOTE: this parsing only support `expiry` specified in hours (e.g. "360h").
SERVER_CERT_LIFESPAN=$(jq '.signing.profiles.server.expiry | scan("[0-9]+") | tonumber | . / 24' ./ca-config.json)

echo "Removing expired certificates:"
find tls -type f -not \( -name 'ca*' -or -name '*-key.pem' -or -name '*.csr' \) -mtime "+$SERVER_CERT_LIFESPAN" -print -delete

for service in parsec s3 proxy; do
    if [ ! -f "tls/${service}.csr" ] || [ "${service}-csr.json" -nt "tls/${service}.csr" ]; then
        cfssl genkey \
            -config=ca-config.json -profile=server "${service}-csr.json" | cfssljson -bare "tls/${service}"
    fi

    if [ ! -f "tls/${service}.pem" ] || [ "tls/${service}.csr" -nt "tls/${service}.pem" ] || [ tls/ca-key.pem -nt "tls/${service}.pem" ]; then
        cfssl sign \
            -ca=tls/ca.pem -ca-key=tls/ca-key.pem \
            -config=ca-config.json -profile=server "tls/${service}.csr" | cfssljson -bare "tls/${service}"
    fi
done
