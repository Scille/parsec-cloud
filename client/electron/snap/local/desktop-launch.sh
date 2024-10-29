#!/bin/bash -e

# Capture the current environment for the future xdg-open calls.
XDG_OPEN_ENVIRONMENT=$(export; echo 'xdg-open "$@"')
export XDG_OPEN_ENVIRONMENT

# Snap bin directory needs more priority than `/usr/bin` and `/bin` in order for
# the `xdg-open` script to work properly.
export PATH="$SNAP/bin:$PATH"

# Run the app through `desktop-launch-wrapper` to populate the environment with
# variables that are specific to the snap environment.
exec "$SNAP/bin/desktop-launch-wrapper" "$SNAP/app/parsec" "$@" --no-sandbox
