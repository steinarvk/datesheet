#!/bin/bash
set -euo pipefail

version=$(git describe --always --abbrev=0 --dirty)
image=${DOCKER_IMAGE}
scriptpath="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

DOCKER_BUILDKIT=1 docker build -t "${image}:${version}" "${scriptpath}/.." -f "${scriptpath}/../Dockerfile"
docker push "${image}:${version}"
echo "${image}:${version}"
