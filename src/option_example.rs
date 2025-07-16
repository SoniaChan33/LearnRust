fn main() {
    // Option学习
}

fn option_example() {
    // 创建Option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    //使用
    let x = plus_one(some_number);
    let y = plus_one(absent_number);
    println!("x: {:?}, y: {:?}", x, y);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn unwrap_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number.unwrap());
    println!("some_string: {:?}", some_string.unwrap());
    // println!("absent_number: {:?}", absent_number.unwrap());
}

fn is_some_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number.is_some());
    println!("some_string: {:?}", some_string.is_some());
    println!("absent_number: {:?}", absent_number.is_some());
}

fn is_none_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number.is_none());
    println!("some_string: {:?}", some_string.is_none());
    println!("absent_number: {:?}", absent_number.is_none());
}

