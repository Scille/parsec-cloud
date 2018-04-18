#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )/..
black --line-length=100 \
  $BASEDIR/parsec \
  $BASEDIR/tests \
  $BASEDIR/setup.py \
  $BASEDIR/examples \
  $@
