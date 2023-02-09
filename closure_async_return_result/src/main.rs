use std::future::Future;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let a = 10;
    let b = 11;

    let inline_async_closure = || async {
        println!("inside inline_async_closure");
        sleep(Duration::from_millis(2000)).await;
        if a > 100 || b > 100 {
            Ok(a == b)
        } else {
            Err(String::from("Some error"))
        }
    };
    println!("inside main {}", bool_string(inline_async_closure).await);
}

async fn bool_string<F>(f: impl FnOnce() -> F) -> String
where
    F: Future<Output = Result<bool, String>>,
{
    let result = f().await;
    let r = if let Ok(equality) = result {
        match equality {
            true => "true",
            _ => "false",
        }
    } else {
        "large number"
    };
    r.to_string()
}
