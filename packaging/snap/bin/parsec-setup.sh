#!/bin/sh

check_env_var_path() {
    set -u
    local type=$1
    local env_name=$2

    case $type in
        file)
        test=-f
        ;;
        directory)
        test=-d
        ;;
        *)
        echo "Unsupported type $type" >&2
        exit 1
        ;;
    esac

    local env_value=$(eval "echo \$$env_name")
    if [ ! $test $env_value ]; then
        echo "$env_name does not point to a $type ($env_value)" >&2
        exit 1
    fi
}

# SNAP env var is generated using the following format `<SNAP_ROOT_DIR>/<APP_NAME>/<APP_VERSION>`
export SNAP_ROOT_DIR=$(realpath ${SNAP}/../..)

check_env_var_path directory SNAP_ROOT_DIR

export CORE20_PATH=$(realpath ${SNAP_ROOT_DIR}/core20/current)

check_env_var_path directory CORE20_PATH

# Expose /sbin and /bin as they are not set in snap classic mode
# In particular, this is useful to access `/bin/fusermount` which is used by libfuse
# We also override the original path as we should not rely on tools installed on the system
# From this point on, we should avoid using tools from the system.
export PATH=${SNAP}/sbin:${SNAP}/bin:${SNAP}/usr/bin:${CORE20_PATH}/bin:${CORE20_PATH}/sbin:${PATH}

# Select the python provided by snap and set the right python path
export PYTHON=${SNAP}/bin/python3.11
export PYTHONPATH=${SNAP}/site-packages

# We can't rely on `ctypes.util.find_library` in classic snap environment,
# so we use the `*_LIBRARY_PATH` environment variables.
export FUSE_LIBRARY_PATH=${SNAP}/lib/x86_64-linux-gnu/libfuse.so.2
# We directly use the provide ssl lib by `core20`.
export SSL_LIBRARY_PATH=${CORE20_PATH}/lib/x86_64-linux-gnu/libssl.so.1.1
# We directly use the provide crypto lib by `core20`.
export CRYPTO_LIBRARY_PATH=${CORE20_PATH}/lib/x86_64-linux-gnu/libcrypto.so.1.1

check_env_var_path file FUSE_LIBRARY_PATH
check_env_var_path file SSL_LIBRARY_PATH
check_env_var_path file CRYPTO_LIBRARY_PATH
