#!/bin/bash
# gen-gha-env.sh
# Copyright (C) 2023
# Squidpie
echo `services/chat/env.rb` > .env
echo "UID=`id -u`" >> .env
echo "GUID=`id -g`" >> .env
