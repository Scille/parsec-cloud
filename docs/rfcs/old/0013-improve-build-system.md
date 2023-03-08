# Improve build system

From [ISSUE-1444](https://github.com/Scille/parsec-cloud/issues/1444)

Current build system relies on `setup.py` (i.e. setuptools) which is fine for regular Python project, but it getting too limited for Parsec:

- `setup_requires` (i.e. dependencies needed for building the project, not for running it) contains PyQt5 and Babel which are only needed for a non-mandatory part (so installing from the source the project on a server requires PyQt5 and Babel which are non-trivial dependencies)
- we have a `pre-requirements.txt` to update setuptools to support the new manylinux2014 platform tag (needed to install PyQt5). This cannot be done is the `setup_requires` field of setup.py` given setuptools is loaded when setup.py starts to be executed.
- extra requirements is generally poorly handled by tools such pip-tools (which prevent us from pinning our dependencies in a lock file), snapcraft (we have to patch setup.py by hand...) and python-for-android

On top of that we have multiple targets to build:

- dev mode: use `setup.py` to install the project in place
- pypi wheel: use `setup.py bdist_wheel sdist`, provides a wheel that will be installed by 3rd party (potentially with only a subset of the extras)
- snap: use `setup.py` to install the project
- win32: use `setup.py` to generate the wheel, then install it
- Android: work in progress, but it's basically a big minefield :'-(

So I'm starting to consider using a more powerful build system such as ~~autotools~~ scons :trollface:

The idea would be to have the user creating a virtualenv with Scons (typically by using the `pre-requirements.txt`), then handling everything else with scons targets:

```shell
scons dev  # install dev dependencies and execute babel&pyqt cooking tasks
scons tests # basically an alias to `py.test tests`, but ensure `scons dev` has been run first
scons build-snap
scons build-win32
scons build-win64
scons build-wheel
```

The good thing is we could move all the weird cooking code we have in `setup.py` (generate PyQt5 resources and forms, extract translations) into the scons build system (given Scons is written in Python this shouldn't be too complicated).
On top of that we could generate&use a custom `setup.py` for each usecase (which would allow us to get rid of the infamous extra_requirements, except for the `setup.py` used to generate the wheel targeting pypi).
As a matter of fact we could even replace the `setup.py` by a `setup.cfg` (or a `pyproject.toml`) to end-up with a simple declarative format file.

The dependencies handling could be done in the build system with the help of pip-tools. Typically we could have a `scons generate-requirements extra=core,backend` command that would output a requirements.txt file ready for any use (consumed by python-for-android, processed by `pip-compile` to generate a lock file with hashes)

Another good thing is the snap/win32/android packaging involve file copy which are currently done in the `.azure-pipelines.yml`, moving this in the build system would simplify the build outside of the CI and allow a better control (typically we don't want to include the resources that have been used by PyQt5 to generate `_resource.py`).

I'm not sure if we should create separate virtualenv for the dev mode or just install the dependencies in the main virtualenv.
Scons as [basically no dependencies](https://github.com/SCons/scons/blob/d8d6322ee1a17518c8b959349385ef85da496793/setup.cfg#L43-L47) so there should be clashed with Parsec dependencies.
The main issue I guess is what should happen when the requirements changes (typically when we switch from one branch into another) automatically detecting that and updating the virtualenv seems like a cool feature, but it's often overkill for simple tests (especially when going back and forth between two branches)
A possibility would be to add an optional `venv=/path/to/my/venv` option in the snap command to force the use of a custom venv instead without trying to update it prior to the run.
