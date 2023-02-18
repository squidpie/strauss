#!/usr/bin/env ruby
# run.rb
# Copyright (C) 2023
# Squidpie

require 'redis'
require 'json'

$ch = "#strauss-chat-test";
$test_msg="0xdeadbeef"

redis = Redis.new(url: "redis://localhost:6379")

def client(redis)
  begin
    redis.subscribe($ch) do |on|
      on.subscribe do |channel, subscriptions|
        puts "::TEST OUTPUT:: Subscribed to #{channel} (#{subscriptions} subscriptions)"
        STDOUT.flush
      end

    on.message do |channel, msg|
      puts "::TEST OUTPUT:: Received #{channel} - [#{msg}]"
      if msg != $test_msg
        puts "::TEST OUTPUT:: REDIS RECEIVE TEST FAILED - #{msg}"
        STDOUT.flush
        exit(-1)
      end
      exit(0)
    end
  end
rescue=>error
  puts "::TEST OUTPUT:: REDIS SUBSCRIBE TEST FAILED - #{error}"
  STDOUT.flush
  exit(-1)
end
end

def publisher(redis)
  begin
    redis.publish($ch, $test_msg)
  rescue=>error
    puts "::TEST OUTPUT:: REDIS PUBLISH TEST FAILED - #{error}"
    STDOUT.flush
    exit(-1)
  end
end

client = Thread.new { client(redis) }
sleep(0.1)
publisher = Thread.new { publisher(redis) }
publisher.join()
client.join()

