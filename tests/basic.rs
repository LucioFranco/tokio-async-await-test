#![feature(pin, async_await, await_macro, futures_api)]

extern crate futures;
extern crate tokio;
extern crate tokio_async_await_test;

use tokio_async_await_test::{async_current_thread_test, async_test};

#[async_test]
async fn basic() {
    await!(example_async_fn());
}

#[async_test]
async fn basic_with_spawn() {
    tokio::spawn_async(
        async {
            await!(example_async_fn());
        },
    );

    await!(example_async_fn());
}

#[async_current_thread_test]
async fn basic_current_thread() {
    await!(example_async_fn());
}

#[async_current_thread_test]
async fn basic_current_thread_with_spawn() {
    tokio::spawn_async(
        async {
            await!(example_async_fn());
        },
    );

    await!(example_async_fn());
}

async fn example_async_fn() {
    assert!(true);
}
