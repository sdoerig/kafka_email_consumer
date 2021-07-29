/**
 * Copyright 2020 Confluent Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;

use std::boxed::Box;
use tokio::runtime::current_thread::Runtime;
use futures::executor::block_on;

mod email_client;
mod utils;

use email_client::EmailClient;

async fn echo_message<M: Message>(msg: M, email_client: &EmailClient) -> Result<(), std::str::Utf8Error> {
    let deserialize = |o| match o {
        None => Ok(""),
        Some(val) => Ok(std::str::from_utf8(val)?),
    };
    let res_string = format!(
        "Consumed record from topic {} partition [{}] @ offset {} with key {} and value {}",
        msg.topic(),
        msg.partition(),
        msg.offset(),
        deserialize(msg.key())?,
        deserialize(msg.payload())?,
    );
    email_client.testmail(&res_string).await;
    println!("{}", &res_string);

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (topic, mut config, email_config) = utils::get_config()?;
    let consumer: StreamConsumer = config.set("group.id", "rust_example_group_1").create()?;
    let email_client = EmailClient::new(email_config);
    consumer.subscribe(&[&topic])?;

    let processor = tokio::prelude::Stream::for_each(tokio::prelude::Stream::filter_map(consumer
        .start(), |result| match result {
            Ok(_) => result.ok(),
            Err(err) => {
                eprintln!("error consuming from message stream: {}", err);
                None
            }
        }), |msg| block_on(echo_message(msg, &email_client)).map_err(|_| eprintln!("error deserializing message")));

    Runtime::new()?
        .block_on(processor)
        .map_err(|_| eprintln!("error running consumer on current thread"))
        .ok();

    Ok(())
}
