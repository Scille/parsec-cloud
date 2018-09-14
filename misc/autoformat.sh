#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )
cd $BASEDIR/..

black --line-length=100 \
  --exclude='parsec/core/gui/ui/' \
  --exclude='parsec/core/gui/resources_rc.py' \
  parsec \
  tests \
  setup.py \
  examples \
  $@
