#!/bin/bash

cd "docs"

if [ "$?" != "0" ]; then
    echo "Launch this script from the root (ie ./misc/gen_doc_translations.sh)"
    exit 1
fi

make gettext

sphinx-intl -c conf.py update -p _build/locale -l fr
