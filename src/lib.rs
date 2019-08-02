//! This is a simple crate that provides a procedural macro similar to `#[test]` that will run the test as a single future on a tokio runtime.
//!
//! # Usage
//!
//! First, you must be on nightly rust as of `2019-02-15`. Add the crate to your `Cargo.toml`.
//!
//! ``` toml
//! [dependencies]
//! tokio-async-await-test = "0.1"
//! ```
//!
//! This will give you the crate but you will also need to make sure that you also have
//! `futures-preview` and `tokio` as dependencies like so.
//!
//! ``` toml
//! futures-preview = { version = "0.3.0-alpha.17" }
//! ```
//!
//! Once, you have all these dependencies you can then use the attribute like so.
//!
//! ``` rust
//! #![feature(async_await)]
//!
//! extern crate futures;
//! extern crate async_await_test;
//!
//! use async_await_test::async_test;
//!
//! #[async_test]
//! async fn basic() {
//!     example_async_fn().await;
//! }
//! ```

#![feature(async_await)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

/// Run a future as a test, this expands to calling the `async fn` via `Runtime::block_on`.
#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use futures::executor;

            #input
            executor::block_on(#test_case_name())
        }
    };

    TokenStream::from(expanded)
}
