import logging
import logstash

test_logger = logging.getLogger('python-logstash-logger')
test_logger.setLevel(logging.INFO)
test_logger.addHandler(logstash.AMQPLogstashHandler(host='localhost', version=1))

test_logger.info('python-logstash: test logstash info message.')

try:
    1/0
except:
    test_logger.exception('python-logstash-logger: Exception with stack trace!')