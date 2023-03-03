# remove-async-await

A procedural macro to make an async function blocking by removing async and awaits. Useful for crates with practically identical blocking and async implementations, aside from having to use `.await`
on some function calls.

## Adding as a dependency

It is recommended to point the dependency to 1.0 so you get any bug fixes I have to make:

```toml
[dependencies]
remove-async-await = "1.0"
```

## Example

This example assumes you want to keep your async API behind an optional feature called `async`.

```rs
#[cfg_attr(not(feature = "async"), remove_async_await::remove_async_await)]
async fn get_string() -> String {
    "hello world".to_owned()
}

#[cfg_attr(not(feature = "async"), remove_async_await::remove_async_await)]
pub async fn print() {
    let string = get_string().await;
    println!("{}", string);
}
```

In this example, if the `async` feature is not used, it would expand to this:

```rs
fn get_string() -> String {
    "hello world".to_owned()
}

pub fn print() {
    let string = get_string();
    println!("{}", string);
}
```

However, if the `async` feature is used, the code will be unaffected.

You can find more examples in the [`tests/` directory](https://github.com/naturecodevoid/remove-async-await/tree/main/tests).

## `remove_async_await_string`

There are 2 macros this library provides:

1. `remove_async_await`: The one you should almost always use. Uses `syn` to parse rust code and remove async from functions and await from expressions. Currently, it can only take a function as an
   input.
2. `remove_async_await_string`: You should only use this one if `remove_async_await` doesn't work for your use case. This is the "dumb macro"; it
   [literally just removes all occurrences of `async` and `.await` from the string representation of the input](https://github.com/naturecodevoid/remove-async-await/blob/main/src/lib.rs#L192). This
   means that while it might work with things other than functions, **you shouldn't use it because if a function or variable name contains "async" or ".await", your code will break.**

## Known issues

Here is a list of known issues/limitations that I probably won't fix (PRs are welcome!):

-   **Issue**: `.await` is not removed when calling a macro

    **Workarounds**:

    -   Move the expression using `.await` to a local variable.

        Example:

        ```rs
        #[remove_async_await::remove_async_await)]
        async fn issue() {
            println!("{}", get_string().await); // `.await` will not be removed
        }

        #[remove_async_await::remove_async_await)]
        async fn workaround() {
            let string = get_string().await; // `.await` **will** be removed
            println!("{}", string);
        }
        ```

    -   Use [`remove_async_await_string`](#remove_async_await_string) (read docs for more info, such as potential bad side effects)

        Example:

        ```rs
        #[remove_async_await::remove_async_await)]
        async fn issue() {
            println!("{}", get_string().await); // `.await` will not be removed
        }

        #[remove_async_await::remove_async_await_string)]
        async fn workaround() {
            println!("{}", get_string().await); // `.await` **will** be removed
        }
        ```

If you want me to add an issue to this list (or fix the issue), please [create a GitHub issue](https://github.com/naturecodevoid/remove-async-await/issues/new)!
