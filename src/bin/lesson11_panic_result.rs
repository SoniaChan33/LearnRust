#[test]
fn tets_panic_example() {
    panic!("这是一个主动 panic");

    let s: Option<String> = None;
    let _ = s.unwrap(); // 触发 panic

    let v = vec![1, 2, 3];
    println!("{}", v[99]); // 越界访问触发 panic

    //RUST_BACKTRACE=1 cargo run 可以通过设置环境变量来获取 panic 的调用栈信息
}

#[test]
fn result_example() {
    match divide(19, 0) {
        Ok(result) => println!("result={}", result),
        Err(err) => println!("{}", err),
    };
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为0".to_string())
    } else {
        Ok(a / b)
    }
}

use std::fs::File;
use std::io::ErrorKind;
fn error_kind_example() {
    let greeting = File::open("hello.txt");

    let greeting_result = match greeting {
        Ok(result) => result,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => File::create("hello.txt").unwrap(),
            other_error => panic!("Error: {}", err),
        },
    };
}

fn main() {}