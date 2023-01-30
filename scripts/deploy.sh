#!/bin/bash
# deploy.sh
# Copyright (C) 2023
# Squidpie

docker compose -f docker-compose.yml -f docker-compose.prod.yml pull && \
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d --remove-orphans && \
docker image prune -f
