# Note : pyinstaller doesn't work with python 3.8 yet.
# see https://github.com/pyinstaller/pyinstaller/issues/4311

cd ../..
python3 setup.py bdist_wheel
python3 -m pip install .
python3 -m pip install 'pyinstaller==4.0'
cd packaging/macOS

if [ $1 == '-f' -o $1 == '--force' ]
then
    FORCE="--noconfirm";
else
    FORCE="";
fi

pyinstaller parsec.spec $FORCE
rm dist/parsec # Delete ununsed binary file

if [[ $1 == '-i' || $1 == '--install' || $2 == '-i' || $2 == '--install' ]]
then
    rm -Rf /Applications/parsec.app;
    echo  "\033[1;33mRemoved previous app version"
    cp -R dist/parsec.app /Applications;
    echo  "\033[1;32mInstalled new version"
fi
