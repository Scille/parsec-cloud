#!/bin/bash -e
# Snap bin directory needs more priority than `/usr/bin` and `/bin` in order for
# the `xdg-open` script to work properly.
export PATH="$SNAP/bin:$PATH"
exec "$SNAP/bin/desktop-launch-wrapper" "$SNAP/app/parsec" "$@" --no-sandbox
