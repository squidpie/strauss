#!/usr/bin/env ruby
# run.rb
# Copyright (C) 2023
# Squidpie

require 'redis'
require 'json'

$rx = "#strauss-chat-msg-rx";
$tx = "#strauss-chat-msg-tx";
$test_msg="meow meow meow";

redis = Redis.new(url: "redis://redis:6379")

def client(redis)
    begin
        puts "::TEST OUTPUT:: Starting Chat Integration Test";
        redis.subscribe($rx) do |on|
            on.subscribe do |channel, subscriptions|
                puts "::TEST OUTPUT:: Subscribed to #{channel} (#{subscriptions} subscriptions)";
            end

            on.message do |channel, msg|
                msg = JSON.parse(msg)['message_text']
                puts "::TEST OUTPUT:: Received #{channel} - [#{msg}]";
                if msg != $test_msg
                    puts "::TEST OUTPUT:: CHAT TEST FAILED - #{msg}";
                    exit(-1)
                end
                exit(0)
            end
        end
    rescue=>error
        puts "::TEST OUTPUT:: REDIS SUBSCRIBE FAILED - #{error}";
        exit(-1)
    end
end

def publisher(redis)
    begin
        puts "::TEST OUTPUT:: Sending Test Chat Message";
        redis.publish($tx, $test_msg)
    rescue=>error
        puts "::TEST OUTPUT:: REDIS PUBLISH FAILED - #{error}";
        exit(-1)
    end
end

client = Thread.new { client(redis) }
sleep(0.1)
publisher = Thread.new { publisher(redis) }
publisher.join()
client.join()

