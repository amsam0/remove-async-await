//! # remove-async-await
//!
//! A procedural macro to make an async function blocking by removing async and awaits. Useful for crates with practically identical blocking and async implementations, aside from having to use `.await`
//! on some function calls.
//!
//! ## Adding as a dependency
//!
//! It is recommended to point the dependency to 1.0 so you get any bug fixes I have to make:
//!
//! ```toml
//! [dependencies]
//! remove-async-await = "1.0"
//! ```
//!
//! ## Example
//!
//! This example assumes you want to keep your async API behind an optional feature called `async`.
//!
//! ```rs
//! #[cfg_attr(not(feature = "async"), remove_async_await::remove_async_await)]
//! async fn get_string() -> String {
//!     "hello world".to_owned()
//! }
//!
//! #[cfg_attr(not(feature = "async"), remove_async_await::remove_async_await)]
//! pub async fn print() {
//!     let string = get_string().await;
//!     println!("{}", string);
//! }
//! ```
//!
//! In this example, if the `async` feature is not used, it would expand to this:
//!
//! ```rs
//! fn get_string() -> String {
//!     "hello world".to_owned()
//! }
//!
//! pub fn print() {
//!     let string = get_string();
//!     println!("{}", string);
//! }
//! ```
//!
//! However, if the `async` feature is used, the code will be unaffected.
//!
//! You can find more examples in the [`tests/` directory](https://github.com/naturecodevoid/remove-async-await/tree/main/tests).
//!
//! ## `remove_async_await_string`
//!
//! There are 2 macros this library provides:
//!
//! 1. `remove_async_await`: The one you should almost always use. Uses `syn` to parse rust code and remove async from functions and await from expressions. Currently, it can only take a function as an
//!    input.
//! 2. `remove_async_await_string`: You should only use this one if `remove_async_await` doesn't work for your use case. This is the "dumb macro"; it
//! [literally just removes all occurrences of `async` and `.await` from the string representation of the input](https://github.com/naturecodevoid/remove-async-await/blob/main/src/lib.rs#L192). This
//! means that while it might work with things other than functions, **you shouldn't use it because if a function or variable name contains "async" or ".await", your code will break.**
//!
//! ## Known issues
//!
//! Here is a list of known issues/limitations that I probably won't fix (PRs are welcome!):
//!
//! -   **Issue**: `.await` is not removed when calling a macro
//!
//!     **Workarounds**:
//!
//!     -   Move the expression using `.await` to a local variable.
//!
//!         Example:
//!
//!         ```rs
//!         #[remove_async_await::remove_async_await)]
//!         async fn issue() {
//!             println!("{}", get_string().await); // `.await` will not be removed
//!         }
//!
//!         #[remove_async_await::remove_async_await)]
//!         async fn workaround() {
//!             let string = get_string().await; // `.await` **will** be removed
//!             println!("{}", string);
//!         }
//!         ```
//!
//!     -   Use [`remove_async_await_string`](#remove_async_await_string) (read docs for more info, such as potential bad side effects)
//!
//!         Example:
//!
//!         ```rs
//!         #[remove_async_await::remove_async_await)]
//!         async fn issue() {
//!             println!("{}", get_string().await); // `.await` will not be removed
//!         }
//!
//!         #[remove_async_await::remove_async_await_string)]
//!         async fn workaround() {
//!             println!("{}", get_string().await); // `.await` **will** be removed
//!         }
//!         ```
//!
//! If you want me to add an issue to this list (or fix the issue), please [create a GitHub issue](https://github.com/naturecodevoid/remove-async-await/issues/new)!

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    fold::{self, Fold},
    Expr, ExprBlock, ItemFn, TraitItemMethod,
};

struct RemoveAsyncAwait;

impl Fold for RemoveAsyncAwait {
    fn fold_item_fn(&mut self, mut i: ItemFn) -> ItemFn {
        // remove async functions
        i.sig.asyncness = None;
        fold::fold_item_fn(self, i)
    }

    fn fold_trait_item_method(&mut self, mut i: TraitItemMethod) -> TraitItemMethod {
        // remove async trait methods
        i.sig.asyncness = None;
        fold::fold_trait_item_method(self, i)
    }

    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            // remove await
            Expr::Await(e) => self.fold_expr(*e.base),
            // remove async blocks
            Expr::Async(e) => self.fold_expr(Expr::Block(ExprBlock {
                attrs: e.attrs,
                label: None,
                block: e.block,
            })),
            _ => fold::fold_expr(self, e),
        }
    }
}

#[proc_macro_attribute]
/// Please see crate level documentation for usage and examples.
pub fn remove_async_await(_args: TokenStream, input: TokenStream) -> TokenStream {
    #[cfg(feature = "debug")]
    {
        println!();
        println!("Input: {}", input.to_string());
    }

    macro_rules! to_token_stream {
        ($input: expr) => {{
            #[cfg(feature = "debug")]
            {
                println!();
                println!("Parsed input: {:#?}", input);
                println!();
            }
            TokenStream::from($input.to_token_stream())
        }};
    }

    // Attempt to parse as ItemFn, then TraitItemMethod, and finally fail
    let output = match syn::parse::<ItemFn>(input.clone()) {
        Ok(item) => to_token_stream!(RemoveAsyncAwait.fold_item_fn(item)),
        Err(_) => match syn::parse::<TraitItemMethod>(input.clone()) {
            Ok(item) => to_token_stream!(RemoveAsyncAwait.fold_trait_item_method(item)),
            Err(_) => TokenStream::from(quote! {
                compile_error!("remove_async_await currently only supports functions and trait methods. if you are using it on a supported type, parsing probably failed; please ensure the input is valid Rust.")
            }),
        },
    };

    #[cfg(feature = "debug")]
    {
        println!();
        println!("Output: {}", output.to_string());
        println!();
    }

    output
}

#[proc_macro_attribute]
/// Please see crate level documentation for usage and examples. (Specifically the `remove_async_await_string` section)
pub fn remove_async_await_string(_args: TokenStream, input: TokenStream) -> TokenStream {
    #[cfg(feature = "debug")]
    {
        println!();
        println!("Input: {}", input.to_string());
    }

    let input = input.to_string();

    let output = input.replace("async", "").replace(".await", "");
    let output: TokenStream = output.parse().unwrap();

    #[cfg(feature = "debug")]
    {
        println!();
        println!("Output: {}", output.to_string());
        println!();
    }

    output
}
