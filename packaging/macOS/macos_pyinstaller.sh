#! /bin/sh

# Note : pyinstaller doesn't work with python 3.8 yet.
# see https://github.com/pyinstaller/pyinstaller/issues/4311

BASEDIR=$(dirname $(greadlink -f "$0") )
cd $BASEDIR/../..
python3 -m pip install 'pyinstaller==4.0'
# Pyinstaller read the imports in the codebase recursively then copy the
# corresponding needed packages from the current virtualenv.
python3 -m pip install[core]
cd $BASEDIR

if [ $1 == '-f' -o $1 == '--force' ]
then
    FORCE="--noconfirm";
else
    FORCE="";
fi

rm -Rf $BASEDIR/build $BASEDIR/dist
pyinstaller parsec.spec $FORCE

if [[ $1 == '-i' || $1 == '--install' || $2 == '-i' || $2 == '--install' ]]
then
    rm -Rf /Applications/parsec.app;
    echo  "\033[1;33mRemoved previous app version"
    cp -R dist/parsec.app /Applications;
    echo  "\033[1;32mInstalled new version"
fi
