# Dockerfile
# Copyright (C) 2023
# Squidpie
ARG version

FROM debian:bullseye-slim AS base
ARG STRAUSS_BUILD_DIR
WORKDIR /strauss
RUN apt update && apt install -y --no-install-recommends ca-certificates
COPY ${STRAUSS_BUILD_DIR}/chat .

FROM base AS val
RUN apt install -y --no-install-recommends \
    curl \
    sudo \
    tcpdump \
    net-tools \
    netcat \
    procps \
    dnsutils \
    unzip \
    lsof  
COPY scripts/entrypoint.val.sh .
ENTRYPOINT ["/strauss/entrypoint.val.sh", "/strauss/chat"]

FROM base AS prod
ENTRYPOINT ["/strauss/chat"]

FROM ${version} AS final
EXPOSE 6379
