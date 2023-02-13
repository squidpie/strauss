# Dockerfile
# Copyright (C) 2023
# Squidpie

FROM ruby:3.2.0-alpine3.17
RUN bundle config --global frozen 1
WORKDIR /usr/src/app
COPY Gemfile Gemfile.lock ./
RUN bundle install
COPY tests/redis/run.rb .
CMD ["./run.rb"]