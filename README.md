# Tokio Async/Await Test

[![Build Status][s1]][tc] [![Crates.io][s2]][ci] [![docs page][docs-badge]][docs] [![MIT/Apache][s3]][li]

[s1]: https://travis-ci.org/LucioFranco/tokio-async-await-test.svg?branch=master
[s2]: https://img.shields.io/crates/v/tokio-async-await-test.svg
[docs-badge]: https://docs.rs/tokio-async-await-test/badge.svg
[docs]: https://docs.rs/tokio-async-await-test
[s3]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[tc]: https://travis-ci.org/LucioFranco/tokio-async-await-test
[ci]: https://crates.io/crates/tokio-async-await-test/
[li]: https://github.com/LucioFranco/tokio-async-await-test/tree/master/license

This is a simple crate that provides a procedural macro similar to `#[test]` that will run the test as a single future on a tokio runtime.

# Usage

First, you must be on nightly rust as of `12-02-2018`. Add the crate to your `Cargo.toml`.

``` toml
[dependencies]
tokio-async-await-test = "0.1"
```

This will give you the crate but you will also need to make sure that you also have `futures-preview` and `tokio` as dependencies like so.

``` toml
tokio = { version = "0.1", features = ["async-await-preview"] }
futures-preview = { version = "0.3.0-alpha.10", features = ["tokio-compat"] }
```

Once, you have all these dependencies you can then use the attribute like so.

``` rust
#![feature(pin, async_await, await_macro, futures_api)]

extern crate futures;
extern crate tokio;
extern crate tokio_async_await_test;

use tokio_async_await_test::async_test;

#[async_test]
async fn basic() {
    await!(example_async_fn());
}
```

This will spin up a tokio runtime and block on the `basic` function. This generally expands to look like this. Where `fut` is the test future you are running.

``` rust
#[test]
fn basic() {
	// -- snip --
    let mut rt = Runtime::new().unwrap();
	
	rt.block_on(fut().unit_error().boxed().compat()).unwrap();
}
```

You can also use a current thread runtime by importing `use tokio_async_await_test::async_current_thread_test;`.
