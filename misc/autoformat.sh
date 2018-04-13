#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )/..
black \
  $BASEDIR/parsec \
  $BASEDIR/tests \
  $BASEDIR/setup.py \
  $BASEDIR/examples \
  $@
