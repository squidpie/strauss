#!/bin/bash
# gen-dev-env.sh
# Copyright (C) 2023
# Squidpie
echo \
"STRAUSS_ROOT_DIR=$(pwd)
STRAUSS_BUILD_DIR=target/debug
GITVERSION_SEMVER=$(docker run --rm -v "$(pwd):/repo" gittools/gitversion:5.6.6 /repo | \
  python3 -c "import sys, json; print(json.load(sys.stdin)['SemVer'])")" > .env
echo `services/chat/env.rb` >> .env
set -a && source .env && set +a