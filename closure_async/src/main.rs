use std::future::Future;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let a = 10;
    let b = 11;

    let inline_async_closure = || async {
        println!("inside inline_async_closure");
        sleep(Duration::from_millis(100)).await;
        a == b
    };
    println!("inside main {}", bool_string(inline_async_closure).await);
}

async fn bool_string<F>(f: impl FnOnce() -> F) -> String
where
    F: Future<Output = bool>,
{
    let result = f().await;
    let r = match result {
        true => "true",
        _ => "false",
    };
    r.to_string()
}
