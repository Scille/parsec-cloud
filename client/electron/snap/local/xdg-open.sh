#!/bin/bash -e
# Explanation:
# When running the app through `desktop-launch-wrapper`, the environment is populated
# with variables that are specific to the snap enviroment. But when the electron app
# uses `xdg-open` to open an external resource, we want to use the host environment
# (otherwise, the host applications might end up crashing when trying to load libraries
# from the snap environment). This is why we expose an `xdg-open` script with a higher
# priority in the PATH that will call the host `xdg-open` binary (see `command.sh`
# above).
# See: https://github.com/Scille/parsec-cloud/issues/8737
XDG_OPEN=$(which -a xdg-open | grep -v "$SNAP" | head -n 1)
FILTERED_PATH=$(echo -n "$PATH" | sed -e "s#$SNAP/bin:##g")
exec env - LANG="$LANG" DISPLAY="$DISPLAY" PATH="$FILTERED_PATH" "$XDG_OPEN" "$@"
