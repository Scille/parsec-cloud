# shellcheck disable=SC2148
function generate_cert_conf() {
    local name=$1
    local san=$2

    echo "Generating $name.crt.conf"

    cat <<EOF >"tls/${name}.crt.conf"
[req]
distinguished_name = req_dist_name
req_extensions = req_ext
prompt = no

[req_dist_name]
CN = $name

[req_ext]
subjectAltName = $san
EOF
}

function generate_key() {
    echo "Generate key at $1"
    openssl genrsa -out "$1" 4096
}

function generate_certificate_request() {
    local name=$1
    echo "Generate certificate request $name.csr"
    openssl req -batch \
        -new -sha512 -noenc \
        -config "tls/${name}.crt.conf" \
        -key "tls/${name}-key.pem" -out "tls/${name}.csr"
}

function sign_crt_with_ca() {
    local ca_crt=$1
    local ca_key=$2
    local name=$3

    echo "Sign certificate request $name.pem"

    openssl x509 -req -in "tls/$name.csr" \
        -CA "$ca_crt" -CAkey "$ca_key" \
        -extfile "tls/$name.crt.conf" \
        -extensions req_ext \
        -CAcreateserial -out "tls/$name.pem" \
        -days 10 -sha512
}

mkdir -p tls

if [ ! -f tls/ca-key.pem ]; then
    generate_key tls/ca-key.pem
fi

if [ ! -f custom-ca.crt ] || [ custom-ca.key -nt custom-ca.crt ]; then
    echo "Generate a mini Certificate Authority"
    openssl req -batch \
        -x509 -sha512 -nodes -days 10 \
        -subj "/CN=Mini Certificate Authority" \
        -key tls/ca-key.pem -out tls/ca.pem
fi

for service in {s3,parsec,proxy}; do
    if [ ! -f "tls/$service.crt.conf" ]; then
        generate_cert_conf "$service" "DNS:parsec-$service,DNS:*.parsec.localhost,DNS:localhost,IP:127.0.0.1"
    fi

    # Generate key if missing
    if [ ! -f "tls/$service-key.pem" ]; then
        generate_key "tls/$service-key.pem"
    fi

    # Generate a CSR if missing or if key is newer
    if [ ! -f "tls/$service.csr" ] || [ "tls/$service-key.pem" -nt "tls/$service.csr" ] || [ "tls/$service.csr" -ot "tls/$service.crt.conf" ]; then
        generate_certificate_request "$service"
    fi

    # Generate crt if missing or if it's older than the csr or the custom CA
    if [ ! -f "tls/$service.pem" ] || [ "tls/$service.pem" -ot "tls/$service.csr" ] || [ "tls/$service.pem" -ot tls/ca.pem ]; then
        sign_crt_with_ca tls/ca{,-key}.pem "$service"
    fi
done
