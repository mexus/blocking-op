use futures::{stream, Sink, Stream};
use std::time::Instant;
use tokio::runtime::Runtime;
use worker::spawn_worker;

mod worker;

fn producer(items: u64) -> impl Stream<Item = Instant, Error = ()> {
    stream::unfold((), |()| {
        let now = Instant::now();
        log::info!("Generated data = {:?}", now);
        Some(Ok((now, ())))
    })
    .take(items)
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut mt_runtime = Runtime::new().expect("Can't start multithreaded runtime");
    let items_per_task = 3;
    let queue_length = 2;

    let task = producer(items_per_task)
        .select(producer(items_per_task))
        .select(producer(items_per_task));
    let processor = spawn_worker(queue_length)
        .sink_map_err(|_| log::error!("Sending data failed"))
        .send_all(task);
    mt_runtime.block_on(processor).unwrap();
}
