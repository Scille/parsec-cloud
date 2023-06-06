#! /bin/sh
# Execute the script like `bash packaging/testbed-server/build.sh ...`

set -e -o pipefail

# Allow the user to overwrite `SCRIPTDIR` by exporting it beforehand.
SCRIPTDIR=${SCRIPTDIR:=$(dirname $(realpath -s "$0"))}
# Allow the user to overwrite `ROOTDIR` by exporting it beforehand.
ROOTDIR=${ROOTDIR:=$(realpath -s "$SCRIPTDIR/../..")}

CURRENT_DATE=$(date --iso-8601)
CURRENT_VERSION=$(grep -o '^version = .*$' $ROOTDIR/pyproject.toml | sed 's/version = "\(.*\)"$/\1/' | tr '+' '-' )
CURRENT_COMMIT_SHA=$(git rev-parse --short HEAD)

UNIQ_TAG="$CURRENT_VERSION.$CURRENT_DATE-sha.$CURRENT_COMMIT_SHA"

echo "Will create an image \`parsec-testbed-server\` with the following tags:"
echo "- \`latest\`"
echo "- \`$UNIQ_TAG\`"

PREFIX=ghcr.io/scille/parsec-cloud

if [ "$WRITE_ENV" -eq 1 ]; then
    (
        echo "UNIQ_TAG=$UNIQ_TAG"
        echo "TAGS=($UNIQ_TAG latest)"
        echo "PREFIX=$PREFIX"
    ) > parsec-testbed.env
fi

echo "On top of that the image will use the following prefix \`$PREFIX\`"

DOCKER_BUILDKIT=1 docker build \
    -f $SCRIPTDIR/testbed-server.dockerfile \
    -t $PREFIX/parsec-testbed-server:$UNIQ_TAG \
    $ROOTDIR $@

echo "You can now test/use the docker image with:"
echo "docker run --publish 6777:6777 --rm --name=parsec-testbed-server $PREFIX/parsec-testbed-server:$UNIQ_TAG"

echo "Once You have tested that the image is working you can push it with"
echo "for tag in $UNIQ_TAG; do docker push $PREFIX/parsec-testbed-server:\$tag; done"
