#! /bin/sh

BASEDIR=$( dirname `readlink -f $0` )
cd $BASEDIR/..

if ( [ "$#" -eq 0 ] ); then
	isort -rc \
	  parsec \
	  tests \
	  setup.py \
	  misc/initialize_test_organization.py \
	  misc/license_headers.py
fi

black --line-length=100 \
  --exclude='/parsec/core/gui/_resources_rc.py|/parsec/core/gui/ui/' \
  parsec \
  tests \
  setup.py \
  misc/initialize_test_organization.py \
  misc/license_headers.py \
  $@
