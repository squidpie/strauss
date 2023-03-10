#!/usr/bin/env ruby
# run.rb
# Copyright (C) 2023
# Squidpie

require 'redis'
require 'json'

$tx = "#strauss-chat-msg-tx"
$rx = "#strauss-chat-msg-rx"
$test_msg = "meow debug meow"

redis = Redis.new(url: "redis://localhost:6379")

def client(redis)
    begin
        redis.subscribe($rx) do |on|
            on.subscribe do |channel, subscribtions|
                puts "::TEST OUTPUT:: Subscribed to #{channel} (#{subscribtions} subs)"
                STDOUT.flush
            end

on.message do |channel, msg|
    puts "::TEST OUTPUT:: Received #{channel} - [#{msg}]"
    STDOUT.flush
    if msg != $test_msg
        puts "::TEST OUTPUT:: TEST FAILED - MSG MISMATCH"
        STDOUT.flush
        exit(-1)
    end
    exit(0)
end
end
rescue=>error
    puts "::TEST OUTPUT:: SUBSCRIBE FAILED - #{error}"
    STDOUT.flush
    exit(-1)
end
end

def publisher(redis)
    begin
        redis.publish($tx, $test_msg)
    rescue=>error
        puts "::TEST OUTPUT:: PUBLISH FAILED - #{error}"
        STDOUT.flush
        exit(-1)
    end
end

client = Thread.new { client(redis) }
sleep(0.1)
publisher = Thread.new { publisher(redis) }
publisher.join()
client.join()
