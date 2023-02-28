# Oxidation in prod - Release architecture

From [ISSUE-2492](https://github.com/Scille/parsec-cloud/issues/2492)

## Current situation

### Server release

A platform-independent wheel is generated as the first step in the CI, then once all tests have passed it is uploaded to pypi.

Deploying to a PAAS (e.g. Heroku), correspond to create a minimal Python project with `parsec-cloud[backend]` as only dependency.

### Client release

Release code is specific for each platform (Linux/Windows/MacOS).

Typically on Windows release is 1) build the project as a wheel, install it with it dependencies 2) use PyInstaller to bundle all this with a Python distribution 3) generate an installer with NSIS

The approach is pretty similar on Linux (using Snap) an MacOS (PyInstaller only)

## New & old issues

- *issue 1*: The current way of deploying with Pypi doesn't preserve dependency pinning (especially transitive ones). This is bad given it makes older version of parsec hard to install and we have no strong guarantee on what runs on the server :(
- *issue 2*: On top of that we the wheel is going to be platform-dependent now that we ship Rust code within it.
- *issue 3*: Snap tries very hard to reuse the system python & libraries, but it breaks because we run in un-sandboxed mode (insert surprised Pikachu meme here), so we end up with a lot of fragile hack

## Approach 1: Stick with the wheel

### Server

The simplest solution is to keep the wheel system and accept that we are going to provide linux-only.

This reduce the usefulness of providing parsec on pypi (it used to be a valid way of installing parsec client) but it's not that bad given pypi was mostly used for server deployment (an nobody is going to anything other than linux for this use-case !)

However the generation of the wheel itself is a more tricky if we want to be portable given we should use [manylinux](https://github.com/pypa/manylinux) to be compatible with most linux distributions.

Of course we can also go full yolo, just build the wheel on ubuntu and consider it is "fine enough" ;-)

### Client

This should be transparent for the client scripts. Of course issues 2 and 3 still remain.

## Approach 2: PyOxidizer

Another more advanced solution is to use [PyOxidizer](https://pyoxidizer.readthedocs.io/en/latest/index.html) to generate a full binary (including python distribution & parsec dependencies).

The good thing it we should be able to control precisely what should be installed (typically using `poetry export` to generate a `requirements.txt` containing all the pinned dependencies and there hash), so no more issue 1 \o/

On top of that, PyOxidizer provide a full distribution with static linking of it dependencies so we should be able to run it everywhere without compatibility issues. So no need for manylinux, and no more issue 3 given snap ship a single binary with no dependencies \o/

Last but not least, PyOxidizer comes with some tooling for [building packages automatically](https://pyoxidizer.readthedocs.io/en/latest/tugger.html). Among other we should try:

- snap to further simplify issue 3
- macOS given we won't use PyInstaller anymore
- debian so that we can release the server as a .deb which can be easily installed on Heroku

## What should we do then ?

Approach 1) is the simplest one (especially in yolo mode) so I think we should start with this one, while toying around with approach 2) which is more promising in the long run.
