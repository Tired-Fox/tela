extern crate proc_macro;
mod docs;
mod helpers;
mod request;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

use syn::{parse_macro_input, ItemFn};

use request::{request_catch, request_endpoint, CatchArgs, RequestArgs};

macro_rules! request_method {
    ($name: ident) => {
        #[proc_macro_error]
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, function: TokenStream) -> TokenStream {
            let mut args = parse_macro_input!(args as RequestArgs);
            let function = parse_macro_input!(function as ItemFn);
            args.methods = vec![stringify!($name).to_uppercase()];

            request_endpoint(args, function)
        }
    };
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn request(args: TokenStream, function: TokenStream) -> TokenStream {
    request_endpoint(
        parse_macro_input!(args as RequestArgs),
        parse_macro_input!(function as ItemFn),
    )
}

request_method!(get);
request_method!(post);
request_method!(delete);
request_method!(put);
request_method!(options);
request_method!(head);
request_method!(trace);
request_method!(connect);
request_method!(patch);

#[proc_macro_error]
#[proc_macro_attribute]
pub fn catch(args: TokenStream, function: TokenStream) -> TokenStream {
    request_catch(
        parse_macro_input!(args as CatchArgs),
        parse_macro_input!(function as ItemFn),
    )
}
