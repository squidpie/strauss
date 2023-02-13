#!/bin/bash
# package.sh
# Copyright (C) 2023
# Squidpie
if [[ -z ${GITVERSION_SEMVER} ]]; then
    echo "Env Var GITVERSION_SEMVER not set"
    exit -1
fi
RelName="strauss-${GITVERSION_SEMVER}"
RelRoot="packaging"
RelPath="${RelRoot}/${RelName}"

# Setup output dir
if [ -d "${RelPath}" ]; then rm -Rf ${RelPath}; fi
mkdir -p ${RelPath}

# Copy required compose files
cp docker-compose.yml ${RelPath}/
cp docker-compose.prod.yml ${RelPath}/

# Create Deployment Environment
./scripts/gen-prod-env.sh
cp .env ${RelPath}/

# Copy deploy script
cp scripts/deploy.sh ${RelPath}/

# Create Package & Cleanup
tar czf ${RelPath}.tar.gz -C packaging ${RelName}
rm -r ${RelPath}/
