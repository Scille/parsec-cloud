#! /bin/sh
# Execute the script like `bash packaging/testbed-server/build.sh ...`

SCRIPTDIR=$(dirname $(realpath -s "$0"))
ROOTDIR="$SCRIPTDIR/../.."

DOCKER_BUILDKIT=1 docker build \
    -f $SCRIPTDIR/testbed-server.dockerfile \
    -t parsec-testbed-server:latest \
    $ROOTDIR $@
