#![feature(pin, async_await, await_macro, futures_api)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use tokio::runtime::Runtime;
            use futures::future::{FutureExt, TryFutureExt};

            let mut rt = Runtime::new().unwrap();

            #input

            rt.block_on(#test_case_name().unit_error().boxed().compat()).unwrap();
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn async_current_thread_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use tokio::runtime::current_thread::Runtime;
            use futures::future::{FutureExt, TryFutureExt};

            let mut rt = Runtime::new().unwrap();

            #input

            rt.block_on(#test_case_name().unit_error().boxed().compat()).unwrap();
        }
    };

    TokenStream::from(expanded)
}
