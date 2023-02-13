#!/bin/bash
# gen-gha-env.sh
# Copyright (C) 2023
# Squidpie
echo \
"STRAUSS_ROOT_DIR=$(pwd)
STRAUSS_BUILD_DIR=target/release
GITVERSION_SEMVER=${GITVERSION_SEMVER}" > .env
echo `services/chat/env.rb` >> .env