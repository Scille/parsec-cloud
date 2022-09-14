#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )
cd $BASEDIR/..

black --line-length=100 \
    --exclude='/parsec/core/gui/_resources_rc.py|/parsec/core/gui/ui/' \
    parsec \
    tests \
    setup.py \
    misc/license_headers.py \
    $@
