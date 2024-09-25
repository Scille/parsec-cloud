#!/usr/bin/env bash

function generate_cert_conf() {
    local name=$1
    local san=$2

    echo "Generating $name.crt.conf"

    cat << EOF > $name.crt.conf
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

function generate_certificate_request() {
    local name=$1
    echo "Generate certificate request $name.csr"
    openssl req -batch \
        -new -sha512 -noenc -newkey rsa:4096 \
        -config $name.crt.conf \
        -keyout $name.key -out $name.csr
}

function sign_crt_with_ca() {
    local ca_crt=$1
    local ca_key=$2
    local name=$3

    echo "Sign certificate request $name.crt"

    openssl x509 -req -in $name.csr \
        -CA $ca_crt -CAkey $ca_key \
        -extfile $name.crt.conf \
        -extensions req_ext \
        -CAcreateserial -out $name.crt \
        -days 10 -sha512
}

if [ ! -f custom-ca.key ]; then
    echo "Generate a mini Certificate Authority"
    openssl req -batch \
        -x509 -sha512 -nodes -days 10 -newkey rsa:4096 \
        -subj "/CN=Mini Certificate Authority" \
        -keyout custom-ca.key -out custom-ca.crt
fi

for service in parsec-{s3,server}; do
    if [ ! -f $service.crt.conf ]; then
        generate_cert_conf $service DNS:$service,DNS:localhost,IP:127.0.0.1
    fi

    if [ ! -f $service.key ]; then
        generate_certificate_request $service
    fi

    if [ ! -f $service.crt ]; then
        sign_crt_with_ca custom-ca.{crt,key} $service
    fi
done

if [ "$(stat -c %g parsec-server.key)" -ne 1234 ]; then
    echo "Changing group id of parsec-server.key to 1234"
    sudo chown $USER:1234 parsec-server.key
fi

if [ "$(stat -c %a parsec-server.key)" -ne 640 ]; then
    echo "Changing permission of parsec-server.key to 640"
    chmod 640 parsec-server.key
fi
