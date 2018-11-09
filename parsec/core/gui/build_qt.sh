#!/bin/bash

mkdir -p rc/translations

pylupdate5 parsec-gui.pro

for f in `ls tr`;
do
    lrelease -compress "tr/${f}" -qm "rc/translations/${f%.*}.qm"
done
