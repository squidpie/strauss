#!/bin/bash
# package.sh
# Copyright (C) 2023
# Squidpie

RelName="strauss-${GitVersion_SemVer}"
RelPath="packaging/${RelName}"

# Setup output dir
if [ -d "${RelPath}" ]; then rm -Rf ${RelPath}; fi
mkdir -p ${RelPath}

# Copy required compose files
cp docker-compose.yml ${RelPath}/
cp docker-compose.prod.yml ${RelPath}/

# Create Deployment Environment
echo "GITVERSION_SEMVER=${GITVERSION_SEMVER}" > ${RelPath}/.env

# Copy deploy script
cp scripts/deploy.sh ${RelPath}/

tar czf ${RelName}.tar.gz -C packaging ${RelName}
mv ${RelName}.tar.gz ${RelPath}/
