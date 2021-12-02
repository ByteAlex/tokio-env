pub fn main() {
    test();
    test_macro();
}

fn test() {
    tokio_env::start_with(async {
        println!("Test");
    }).expect("Failed to start runtime");
}


fn test_macro() {

    #[tokio_env::main]
    pub async fn test() -> i32 {
        let task = tokio::spawn(async move {
            42
        });
        println!("Awaiting answer to the universe");
        task.await.expect("Failed to await task")
    }

    let result = test();
    println!("Answer is {}", result);
}