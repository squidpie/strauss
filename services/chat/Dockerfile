# Dockerfile
# Copyright (C) 2023
# Squidpie
ARG version

FROM debian:bullseye-slim AS base
ARG STRAUSS_BUILD_DIR
WORKDIR /strauss
RUN apt update && apt install -y ca-certificates libffi7
COPY ${STRAUSS_BUILD_DIR}/chat .

FROM base AS val
RUN apt install -y curl
RUN echo "CONFIGURING VAL PROXY"
ENV http_proxy="http://doe:p@ssw0rd@mockserver:1090/"
ENV https_proxy="http://doe:p@ssw0rd@mockserver:1090/"
ENV ftp_proxy="http://doe:p@ssw0rd@mockserver:1090/"
ENV no_proxy="127.0.0.1,localhost,mockserver"
ENV HTTP_PROXY="http://doe:p@ssw0rd@mockserver:1090/"
ENV HTTPS_PROXY="http://doe:p@ssw0rd@mockserver:1090/"
ENV FTP_PROXY="http://doe:p@ssw0rd@mockserver:1090/"
ENV NO_PROXY="127.0.0.1,localhost,mockserver"

FROM base AS prod

FROM ${version} AS final
ENTRYPOINT ["/strauss/chat"]
EXPOSE 6379
