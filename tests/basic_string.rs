#[remove_async_await::remove_async_await_string]
async fn get_string() -> String {
    "hello world".to_owned()
}

#[remove_async_await::remove_async_await_string]
async fn print() {
    println!("{}", get_string().await);
}

#[remove_async_await::remove_async_await_string]
#[test]
async fn basic_string() {
    print().await;

    let result = async {
        println!("goodbye world");
        true
    }
    .await;
    assert_eq!(result, true);
}
