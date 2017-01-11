if [ -n "${OPENSSL}" ]; then
    OPENSSL_DIR="ossl-1/${OPENSSL}"

    export PATH="$HOME/$OPENSSL_DIR/bin:$PATH"
    export CFLAGS="-I$HOME/$OPENSSL_DIR/include"
    # rpath on linux will cause it to use an absolute path so we don't need to do LD_LIBRARY_PATH
    export LDFLAGS="-L$HOME/$OPENSSL_DIR/lib -Wl,-rpath=$HOME/$OPENSSL_DIR/lib"
fi

source ~/.venv/bin/activate
tox -e $(echo py$TRAVIS_PYTHON_VERSION | tr -d .) -- -v $TOX_FLAGS