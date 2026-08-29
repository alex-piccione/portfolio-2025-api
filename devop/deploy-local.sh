#!/bin/bash

# Used for local deployment with Docker Swarm, like in production.
# It builds the image and than use it in the Swarm.

set -a  # automatically export all variables
source .env
set +a

export MSYS_NO_PATHCONV=1  # Disable path conversion (otherwise on GitBash in Windows /data becomes C:/data which is not desired here)

docker build -t portfolio-api:local-1 -f ../Dockerfile ../ --build-arg CONFIGURATION_FILE=${CONFIGURATION_FILE} --build-arg VERSION=${VERSION}
docker stack deploy --prune --detach=false -c docker-stack.yml portfolio-api
# --with-registry-auth is not needed for local images

# docker container prune -f    to cleanup old containers

### ignore this warning (caused by the fact the image it NOT taken from a registry):
# image portfolio-api:local-1 could not be accessed on a registry to record
# its digest. Each node will access portfolio-api:local-1 independently,
# possibly leading to different nodes running different versions of the image.
