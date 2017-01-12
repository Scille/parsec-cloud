#!/bin/bash

set -e
set -x
OLD_DIR=`pwd`
if [ -n "${OPENSSL}" ]; then
    OPENSSL_DIR="ossl-1/${OPENSSL}"
    if [[ ! -f "$HOME/$OPENSSL_DIR/bin/openssl" ]]; then
        curl -O https://www.openssl.org/source/openssl-$OPENSSL.tar.gz
        tar zxf openssl-$OPENSSL.tar.gz
        cd openssl-$OPENSSL
        ./config shared no-asm no-ssl2 no-ssl3 -fPIC --prefix="$HOME/$OPENSSL_DIR"
        make -j4 >/dev/null
        make install >/dev/null
        cd ..
        rm -rf openssl-$OPENSSL
    fi
fi
cd $OLD_DIR
