/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use slog::{o, Discard, Logger};

use crate::{
    client::{self, Client},
    error::ClientError,
    message::Message,
};

pub struct ProducerBuilder {
    target: String,
    log: Logger,
}

impl ProducerBuilder {
    pub fn new(target: &str) -> Self {
        let drain = Discard;
        let root = Logger::root(drain, o!());
        Self {
            target: target.to_owned(),
            log: root,
        }
    }

    pub fn set_log(mut self, log: Logger) -> Self {
        self.log = log;
        self
    }

    pub fn build(self) -> Result<Producer, ClientError> {
        let client = Client::new(self.log, &self.target)?;
        Ok(Producer { client })
    }
}

pub struct Producer {
    client: client::Client,
}

#[derive(Debug, PartialEq)]
pub enum SendReceipt {
    Success { message_id: String },
    Failure { cause: String, target_host: String },
}

impl Producer {
    pub async fn send(&mut self, message: Message) -> Result<SendReceipt, ClientError> {
        Ok(SendReceipt::Success {
            message_id: String::from("abc"),
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{message, util};

    use super::*;

    #[tokio::test]
    async fn test_builder() -> Result<(), ClientError> {
        let target = "localhost:9876";
        let log = util::terminal_logger();
        let mut producer = ProducerBuilder::new(target).set_log(log).build()?;

        let message = message::Message {};
        let send_receipt = producer.send(message).await?;
        assert_eq!(
            SendReceipt::Success {
                message_id: "abc".to_owned(),
            },
            send_receipt
        );
        Ok(())
    }
}
