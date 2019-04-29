//! Single-threaded blocking processor (worker).

use futures::{
    sync::mpsc::{channel, Sender},
    Future, Stream,
};
use std::{
    thread,
    time::{Duration, Instant},
};
use tokio::runtime::current_thread::Runtime;

fn hard_work(data: Instant) {
    let latency = data.elapsed();
    log::info!("Received {:?}, latency = {:?}", data, latency);
    thread::sleep(Duration::from_secs(1));
}

pub fn spawn_worker(queue_length: usize) -> Sender<Instant> {
    let (snd, rcv) = channel(queue_length);
    thread::spawn(move || {
        let mut rt = Runtime::new().expect("Can't setup a current thread runtime");
        let process = rcv
            .for_each(|data| {
                hard_work(data);
                Ok(())
            })
            .then(|_| {
                log::info!("No more senders");
                Ok::<_, ()>(())
            });
        rt.block_on(process).unwrap();
    });
    snd
}
