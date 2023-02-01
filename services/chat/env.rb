#!/usr/bin/env ruby
raw_version = File.open("services/chat/Cargo.toml") do |file|
  file.find { |line| line =~ /^version = .*/ }
end
parsed_version = raw_version.split('=')[1].tr('" ', '')
puts "STRAUSS_CHAT_PKG_VERSION=#{parsed_version}"