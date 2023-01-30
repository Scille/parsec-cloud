#! /bin/sh

cp Dockerfile ../..
cd ../..
DOCKER_BUILDKIT=1 docker build . $@
rm Dockerfile
