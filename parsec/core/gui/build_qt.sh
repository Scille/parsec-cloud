#!/bin/bash

mkdir -p ui
mkdir -p rc/translations

pylupdate5 parsec-gui.pro

for f in `ls forms`;
do
    pyuic5 -o "ui/${f%.*}.py" "forms/${f}" --import-from=parsec.core.gui
done

for f in `ls rc`;
do
    if [ ${f: -4} == ".qrc" ];
    then
        pyrcc5 -o "${f%.*}_rc.py" "rc/${f}"
    fi
done

for f in `ls tr`;
do
    lrelease -compress "tr/${f}" -qm "rc/translations/${f%.*}.qm"
done
