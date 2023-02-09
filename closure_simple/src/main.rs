fn main() {
    let a = 10;
    let b = 11;

    let inline_async_closure = || {
        println!("inside inline_async_closure");
        a == b
    };
    println!("inside main {}", bool_string(inline_async_closure));
}

fn bool_string(f: impl FnOnce() -> bool) -> String {
    let result = f();
    let r = match result {
        true => "true",
        _ => "false",
    };
    r.to_string()
}
