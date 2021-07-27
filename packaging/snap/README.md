Snap installer for Parsec
=========================


Build steps
-----------


### 1 - Generate the Snapcraft build dir


Run the `generate_snap_build_dir.py` Python script with the path to the Parsec sources to use:
```shell
$ python generate_snap_build_dir.py --program-source=../.. --output=./build
```

This will generate a build dir containing Parsec sources, and snapcraft build files
customized with the correct version info.


### 2 - Build the snap

Go to the build dir folder and run the snapcraft command:
```shell
$ cd ./build
$ snapcraft
```

This will generate a `build\parsec-<version>.snap`.
