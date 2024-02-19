use std::collections::HashMap;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::Drain;
extern crate num;
use num::BigUint;
static THREAD_LENGTH: u32 = 3;


fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let (sender, receiver) = mpsc::channel();
    let mut children = Vec::new();
    let logger_cpy = logger.clone();
    let output = Arc::new(RwLock::new(HashMap::<u32, BigUint>::new()));
    let output_cpy = Arc::clone(&output);
    // receiver
    children.push(thread::spawn(move || {
        while let Ok(number) = receiver.recv_timeout(Duration::from_secs(3)) {
            info!(logger_cpy, "Received event-{number}");
            let mut output = output_cpy.write().expect("RwLock poisoned");
            output.insert(number, factorial(number));
            info!(logger_cpy, "Processing event-{number} successful!\n");
        }
        warn!(logger_cpy, "No event emitted in last 10 seconds");
    }));

    // senders
    for number in 0..THREAD_LENGTH {
        let logger_cpy = logger.clone();
        let thread_tx = sender.clone();

        children.push(thread::spawn(move || {
            info!(logger_cpy, "Sending event-{number}...");
            thread_tx.send(number).unwrap();
        }));
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    info!(logger, "Finished!");
    info!(logger, "Factorial output: {:#?}", output.read().unwrap());
}

fn factorial(number: u32) -> BigUint {
    let big_1 = 1u32.into();
    let big_2 = 2u32.into();

    if number < big_2 {
        big_1
    } else {
        let prev_factorial = factorial(number.clone() - 1);
        number * prev_factorial
    }
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1u32.into());
    assert_eq!(factorial(1), 1u32.into());
    assert_eq!(factorial(2), 2u32.into());
    assert_eq!(factorial(3), 6u32.into());
    assert_eq!(factorial(10), 3628800u32.into());
}
