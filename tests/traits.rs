trait TestTrait {
    #[remove_async_await::remove_async_await]
    async fn to_impl(&mut self) -> String;

    #[remove_async_await::remove_async_await]
    async fn default_impl(&mut self) -> String {
        println!("default impl called");
        self.to_impl().await
    }
}

struct TestStruct;

impl TestTrait for TestStruct {
    #[remove_async_await::remove_async_await]
    async fn to_impl(&mut self) -> String {
        "test".to_owned()
    }
}

#[remove_async_await::remove_async_await]
#[test]
async fn traits() {
    let string = TestStruct.default_impl().await;
    println!("{string}");
}
