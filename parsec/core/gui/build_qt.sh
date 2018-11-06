#!/bin/bash

mkdir -p ui
mkdir -p rc/translations

pylupdate5 parsec-gui.pro

for f in `ls forms`;
do
    echo "Processing form ${f}"
    pyuic5 -o "ui/${f%.*}.py" "forms/${f}" --import-from=parsec.core.gui
done

for f in `ls tr`;
do
    lrelease -compress "tr/${f}" -qm "rc/translations/${f%.*}.qm"
done
