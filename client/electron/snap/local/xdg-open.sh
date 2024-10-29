#!/bin/bash -e

# When running the app through `desktop-launch-wrapper`, the environment is populated
# with variables that are specific to the snap environment. But when the electron app
# uses `xdg-open` to open an external resource, we want to use the host environment
# (otherwise, the host applications might end up crashing when trying to load libraries
# from the snap environment). This is why we expose this `xdg-open` script with a higher
# priority in the PATH (see `desktop-launch.sh`).
# See: https://github.com/Scille/parsec-cloud/issues/8737

if [ -z "$XDG_OPEN_ENVIRONMENT" ];
then
    # A fallback in case the environment variable is not set.
    # This should not happen, but better safe than sorry.
    XDG_OPEN=$(which -a xdg-open | grep -v "/snap/" | head -n 1)
    exec "$XDG_OPEN" "$@"
else
    exec env - bash -c "$XDG_OPEN_ENVIRONMENT" -- "$@"
fi
