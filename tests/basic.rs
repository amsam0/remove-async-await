#[remove_async_await::remove_async_await]
async fn get_string() -> String {
    "hello world".to_owned()
}

#[remove_async_await::remove_async_await]
async fn print() {
    let string = get_string().await;
    println!("{}", string);
}

#[remove_async_await::remove_async_await]
#[test]
async fn basic() {
    print().await;

    let result = async {
        println!("goodbye world");
        true
    }
    .await;
    assert_eq!(result, true);
}
