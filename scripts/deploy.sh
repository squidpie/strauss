#!/bin/bash
# deploy.sh
# Copyright (C) 2023
# Squidpie

cat .env > .env.runtime
cat .secrets >> .env.runtime
docker compose --env-file=.env.runtime -f docker-compose.yml -f docker-compose.prod.yml pull && \
docker compose --env-file=.env.runtime -f docker-compose.yml -f docker-compose.prod.yml up -d --remove-orphans && \
docker image prune -f
