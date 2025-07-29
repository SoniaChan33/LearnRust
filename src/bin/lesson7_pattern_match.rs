use std::{arch::aarch64::int32x2x2_t, ops::Add};

#[test]
fn match_example() {
    let x = 5;
    // 必须要有所有可能的case
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    match x {
        1 | 2 => println!("one or two"),
        3 | 4 => println!("three or four"),
        _ => println!("something else"),
    }

    let x = Some(5);
    match x {
        Some(5) => println!("five"),
        _ => println!("something else"),
    }

    enum MyEnum {
        Hello { a: i32 },
        B(String),
        C,
    }
    // 绑定：可以把匹配到的值绑定到一个变量上
    let enum1 = MyEnum::Hello { a: 5 };
    match enum1 {
        MyEnum::Hello { a: val @ 0..=5 } => println!("x is {}", val),
        MyEnum::Hello { a: val @ 5.. } => println!("x is {}", val),
        MyEnum::B(s) => println!("s is {}", s),
        MyEnum::C => println!("C"),
        _ => println!("something else"),
    }

    // 守卫： 在模式匹配中，可以使用守卫来过滤匹配项。
    let x = 5;
    match x {
        n if n > 5 => println!("x is greater than 5"),
        _ => println!("x is less than or equal to 5"),
    }
}

/**
 * 匹配模式应用场景
 */
fn pattern_match_example() {
    // 匹配错误
    match divide(3, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

/**
 * 高级匹配技巧
 */
fn match_advanced_example() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero is not allowed".to_string());
    } else {
        Ok(a / b)
    }
}

#[test]
fn iterator_match_example() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    for (a, b) in vec1.iter().zip(vec2) {
        println!("{} + {} = {}", a, b, a + b);
    }
}

fn if_let_example() {
    let option = Some(5);
    if let Some(x) = option {
        println!("{}", x);
    }
}

fn while_let_example() {
    let mut vec = vec![1, 2, 3];
    while let Some(x) = vec.pop() {
        println!("{}", x);
    }
}

fn ref_example() {
    let mut x = 5;
    match x {
        ref mut var => {
            *var += 1;
        }
        _ => {}
    }
}

fn main() {}
