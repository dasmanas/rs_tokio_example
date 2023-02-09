use std::future::Future;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let a = 10;
    let b = 11;

    let res = call_closure(|| do_something(a, b)).await;
    println!("printing result in main function {}", res);
}

async fn do_something(a: i32, b: i32) -> bool {
    sleep(Duration::from_millis(100)).await;
    a == b
}

async fn call_closure<F>(f: impl FnOnce() -> F) -> bool
where
    F: Future<Output = bool>,
{
    let result = f().await;
    let r = match result {
        true => {
            println!("it is true");
            true
        }
        false => {
            println!("it is false");
            false
        }
    };
    r
}
