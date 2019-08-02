#![feature(async_await)]

extern crate futures;

use tokio_async_await_test::{async_test};

#[async_test]
async fn basic() {
    example_async_fn().await;
}

async fn example_async_fn() {
    assert!(true);
}
