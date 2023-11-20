#!/bin/bash

set -euo pipefail
root_dir=$(realpath $(dirname "${BASH_SOURCE[0]}")/../../zk_backend)

cd "$root_dir"
rm -rf build-result/*
DOCKER_BUILDKIT=1 docker build --no-cache --build-arg BASE_IMAGE=rust:1.68.0 --target build-result --output type=local,dest=build-result .
cp --force build-result/*.so ../services/lib/
