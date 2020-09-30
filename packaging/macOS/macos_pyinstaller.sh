# Note : pyinstaller doesn't work with python 3.8 yet.
# see https://github.com/pyinstaller/pyinstaller/issues/4311

cd ../..
python3 setup.py bdist_wheel
python3 -m pip install .
python3 -m pip install 'pyinstaller==4.0'
cd packaging/macOS
pyinstaller parsec.spec
