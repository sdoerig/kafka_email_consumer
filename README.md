# Overview

Produce messages to and consume messages from a Kafka cluster using the [rust-rdkafka client for Apache Kafka](https://github.com/fede1024/rust-rdkafka). Sending an email for each consumed message. Pruducer and consumer are based on the example at [https://docs.confluent.io/platform/current/tutorials/examples/clients/docs/rust.html](https://docs.confluent.io/platform/current/tutorials/examples/clients/docs/rust.html?utm_source=github&utm_medium=demo&utm_campaign=ch.examples_type.community_content.clients-ccloud)

# Configuration

The configutaion is a key value file:

```
# Kafka
bootstrap.servers=broker_host:9092
security.protocol=SASL_SSL
sasl.mechanisms=PLAIN
sasl.username=username
sasl.password=very_secret
email.from=from_email_address
email.reply_to=reply_to_email_address
email.to=send_to_email_address
starttls_relay=start_tls_relay
smpt_user=smpt_user
smpt_password=smpt_password
```

## Usage

### Producer 

Just for testing

```
./producer --config ~/your.config --topic your_kafka_topic
```

### Consumer 

Sends an Email for each consumed message

```
./consumer --config ~/your.config --topic your_kafka_topic
```
### Build

I had the problem zstd lib was not found when linking so I did
```
export RUSTFLAGS='-C link-args=-lzstd'
cargo build
```