#!/bin/bash
echo \
"STRAUSS_ROOT_DIR=$(pwd)
STRAUSS_BUILD_DIR=$(pwd)/target/release
GITVERSION_SEMVER=${GITVERSION_SEMVER}" > .env
echo `services/chat/env.rb` >> .env