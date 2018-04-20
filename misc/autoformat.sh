#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )
cd $BASEDIR/..

black --line-length=100 \
  parsec \
  tests \
  setup.py \
  examples \
  $@
