use rocketmq::{Message, ProducerBuilder, SendReceipt};
use slog::{o, Drain};
use slog_async::OverflowStrategy;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .overflow_strategy(OverflowStrategy::Drop)
        .build()
        .fuse();
    let log = slog::Logger::root(drain, o!());

    let target = "localhost:9876";
    let mut producer = ProducerBuilder::new(target).set_log(log).build()?;

    let message = Message {};

    if let SendReceipt::Success { message_id } = producer.send(message).await? {
        println!("Send OK. MessageId: `{}`", message_id);
    }

    Ok(())
}
