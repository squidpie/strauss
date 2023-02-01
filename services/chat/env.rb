#!/usr/bin/env ruby
# env.rb
# Copyright (C) 2023
# Squidpie

raw_version = File.open("services/chat/Cargo.toml") do |file|
  file.find { |line| line =~ /^version = .*/ }
end
parsed_version = raw_version.split('=')[1].tr('" ', '')
puts "STRAUSS_CHAT_PKG_VERSION=#{parsed_version}"