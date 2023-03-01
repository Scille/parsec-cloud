#! /bin/bash
# Execute the script line `bash packaging/backend/build.sh ...`

set -e

SCRIPTDIR=$(dirname $(realpath -s "$0"))
ROOTDIR="$SCRIPTDIR/../.."

CURRENT_DATE=$(date --iso-8601)
CURRENT_VERSION=$(grep -o '^version = .*$' $ROOTDIR/pyproject.toml | sed 's/version = "\(.*\)"$/\1/' | tr '+' '.')
CURRENT_COMMIT_SHA=$(git rev-parse --short HEAD)

UNIQ_TAG="$CURRENT_DATE-$CURRENT_VERSION-$CURRENT_COMMIT_SHA"
# We use Github package repository to store our docker's container.
PREFIX=ghcr.io/scille/parsec-cloud

echo "Will create an image \`parsec-backend-server\` with the following tags:"
echo "- \`latest\`"
echo "- \`$UNIQ_TAG\`"
echo
echo "On top of that the image will use the following prefix \`$PREFIX\`"
echo

DOCKER_BUILDKIT=1 docker build \
    -f $SCRIPTDIR/server.dockerfile \
    -t $PREFIX/parsec-backend-server:latest \
    -t $PREFIX/parsec-backend-server:$UNIQ_TAG \
    $ROOTDIR $@

echo
echo "You can now test/use the docker image with:"
echo "docker --port 6777:6777 --env-file env.list --rm --name=parsec-backend-server $PREFIX/parsec-backend-server:$UNIQ_TAG"
echo
echo "Note:"
echo "  You need to configure the env variables in a file to be provided to \`--env-file\`"
echo "  Refer to the example file \`template-prod.env.list\`"
echo
echo "Once You have tested that the image is working you can push it with"
echo "for tag in latest $UNIQ_TAG; do docker push $PREFIX/parsec-backend-server:\$tag; done"
