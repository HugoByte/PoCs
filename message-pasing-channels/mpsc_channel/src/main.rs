use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::Drain;

static THREAD_LENGTH: i32 = 10;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let (sender, receiver) = mpsc::channel();
    let mut children = Vec::new();
    let logger_cpy = logger.clone();

    // receiver
    children.push(thread::spawn(move || {
        while let Ok(event) = receiver.recv_timeout(Duration::from_secs(5)) {
            info!(logger_cpy, "Received event-{event}");
            info!(logger_cpy, "Processing event-{event}");
            sleep(Duration::from_secs(5));
            info!(logger_cpy, "Processing event-{event} successful!\n");
        }
        warn!(logger_cpy, "No event emitted in last 10 seconds");
    }));

    // senders
    for id in 0..THREAD_LENGTH {
        let logger_cpy = logger.clone();
        let thread_tx = sender.clone();

        children.push(thread::spawn(move || {
            info!(logger_cpy, "Sending event-{id}...");
            thread_tx.send(id).unwrap();
        }));
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    info!(logger, "Finished!");
}
