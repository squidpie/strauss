#!/usr/bin/env python
# test.py
# Copyright (C) 2023
# Squidpie
import subprocess
import shlex
from argparse import ArgumentParser, FileType
from configparser import ConfigParser
from confluent_kafka import Producer, Consumer, OFFSET_BEGINNING

POLL_TIMEOUT = 0.25
WATCHDOG_TIMEOUT = 1

def parse_args():
  # Parse the command line.
  parser = ArgumentParser()
  parser.add_argument('config_file', type=FileType('r'))
  parser.add_argument('--reset', action='store_true')
  return parser.parse_args()


def create_config(args):
  # Parse the configuration.
  # See https://github.com/edenhill/librdkafka/blob/master/CONFIGURATION.md
  config_parser = ConfigParser()
  config_parser.read_file(args.config_file)
  config = dict(config_parser['default'])
  config.update(config_parser['consumer'])
  return config


def create_test_topic():
  # Create the test topic
  docker_cmd = "docker exec broker kafka-topics " \
               + "--bootstrap-server broker:9092 " \
               + "--create --topic strausstest" \
               + "--partitions 1"
  subprocess.run(shlex.split(docker_cmd))



def run_producer(config):
  # Create Producer instance
  producer = Producer(config)

  # Optional per-message delivery callback (triggered by poll() or flush())
  # when a message has been successfully delivered or permanently
  # failed delivery (after retries).
  def delivery_callback(err, msg):
    if err:
      print('::TEST OUTPUT:: ERROR: Message failed delivery: {}'.format(err))
      exit(-1)
    else:
      print("::TEST OUTPUT:: Produced event to topic {topic}: key = {key:12} value = {value:12}".format(
        topic=msg.topic(), key=msg.key().decode('utf-8'), value=msg.value().decode('utf-8')))
      global is_delivery_success
      is_delivery_success = True

  producer.produce("strausstest", "0xdeadbeef", "strauss-test", callback=delivery_callback)

  # Block until the messages are sent.
  producer.poll(10000)
  producer.flush()


def run_consumer(config):
  # Create Consumer instance
  consumer = Consumer(config)

  # Set up a callback to handle the '--reset' flag.
  def reset_offset(consumer, partitions):
    if args.reset:
      for p in partitions:
        p.offset = OFFSET_BEGINNING
      consumer.assign(partitions)

  # Subscribe to topic
  topic = "strausstest"
  consumer.subscribe([topic], on_assign=reset_offset)

  # Poll for new messages from Kafka and print them.
  watchdog = WATCHDOG_TIMEOUT * (10/POLL_TIMEOUT)
  msg = consumer.poll(POLL_TIMEOUT)
  if msg is not None and msg.error():
    print("::TEST OUTPUT:: ERROR: %s".format(msg.error()))
    exit(-1)

  print("::TEST OUTPUT:: Waiting for message...")
  while(msg is None and watchdog > 0):
    watchdog-=1
    msg = consumer.poll(POLL_TIMEOUT)
    if msg is not None and msg.error():
      print("::TEST OUTPUT:: ERROR: %s".format(msg.error()))
      exit(-1)

  if (msg is None and watchdog <= 0):
    print("::TEST OUTPUT:: ERROR: watchdog timeout")
    exit(-1)
  # Extract the (optional) key and value, and print.
  print("::TEST OUTPUT:: Consumed event from topic {topic}: key = {key:12} value = {value:12}".format(
    topic=msg.topic(), key=msg.key().decode('utf-8'), value=msg.value().decode('utf-8')))

  consumer.close()


if __name__ == '__main__':
  global is_delivery_success
  is_delivery_success = False
  print("::TEST OUTPUT:: PARSING ARGS")
  args = parse_args()
  config = create_config(args)

  print("::TEST OUTPUT:: CREATING TOPIC")
  create_test_topic()

  print("::TEST OUTPUT:: RUNNING PRODUCER")
  run_producer(config)

  print("::TEST OUTPUT:: RUNNING CONSUMER")
  run_consumer(config)

  print("::TEST OUTPUT:: CHECKING FOR DELIVERY")
  if (not is_delivery_success): exit(-1)
  print("::TEST OUTPUT:: PASSED")
  exit(0)
