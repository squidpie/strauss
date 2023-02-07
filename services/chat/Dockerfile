# Dockerfile
# Copyright (C) 2023
# Squidpie

FROM debian:bullseye-slim
WORKDIR /strauss
RUN apt update && apt install -y ca-certificates
COPY chat .
CMD ./chat
EXPOSE 6379
