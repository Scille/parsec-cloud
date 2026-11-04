#!/usr/bin/env bash
set -eu

# cspell: words objectstore

SLOT_USER_PIN=${SLOT_USER_PIN:=1234}
SLOT_SO_PIN=${SLOT_SO_PIN:=123456}

WORKDIR=${1:?"Need a folder path as argument"}
WORKDIR=$(readlink -f "$WORKDIR")

if [ -d "$WORKDIR" ]; then
    echo "error: '$WORKDIR' already exists" >&2
    exit 1
fi

if ! command -v softhsm2-util &>/dev/null; then
    echo "error: softhsm2-util not found (do `sudo apt install softhsm2` ?)" >&2
    exit 1
fi

mkdir -p "$WORKDIR/softhsm2_tokens"

cat > "$WORKDIR/softhsm2.conf" <<EOF
# SoftHSM v2 configuration file

directories.tokendir = $WORKDIR/softhsm2_tokens/
objectstore.backend = file

# ERROR, WARNING, INFO, DEBUG
log.level = ERROR

# If CKF_REMOVABLE_DEVICE flag should be set
slots.removable = false

# Enable and disable PKCS#11 mechanisms using slots.mechanisms.
slots.mechanisms = ALL

# If the library should reset the state on fork
library.reset_on_fork = false
EOF

init_output=$(SOFTHSM2_CONF="$WORKDIR/softhsm2.conf" softhsm2-util --init-token --slot 0 --label "My token" --pin "$SLOT_USER_PIN" --so-pin "$SLOT_SO_PIN")
echo "$init_output"

SOFTHSM2_SLOT=$(echo "$init_output" | rev | cut -d ' ' -f 1 | rev)
echo ""
echo "Slot ID: $SOFTHSM2_SLOT"
echo ""
echo "To use it:"
echo "  export SOFTHSM2_CONF=$WORKDIR/softhsm2.conf"
echo "  export SOFTHSM2_SLOT=$SOFTHSM2_SLOT"
