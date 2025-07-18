pub fn string_example() {
    let s1 = String::from("Hello, Rust!"); // 第一种创建字符串的方式
    let s2: &str = "Hello, Rust!"; // 第二种创建字符串的方式

    let s3 = String::from("Rust");
    say_hello(&s3); // Rust自动解引用s3，&String会自动转换为&str
    say_hello(s3.as_str()); // 显式转换为&str
    say_hello(&s3[..]); // 通过切片转换为&str

    let mut s = String::from("Hello");
    s.pop(); // 删除最后一个字符
    println!("After pop: {}", s);
    s.remove(0); // 删除第一个字符
    println!("After remove: {}", s);
    s.truncate(3); // 截断字符串到指定长度
    println!("After truncate: {}", s);
    s.clear(); // 清空字符串
    println!("After clear: {}", s);
    // 字符串连接
    let s4 = String::from("Hello");
    let s5 = String::from(", Rust!");
    let s6 = s4 + &s5; // 使用 + 运算符连接字符串，使用 &s5 传递引用
    println!("After concat: {}", s6);
    //format
    let s7 = format!("{}{}", s5, " is awesome!"); // 使用 format! 宏连接字符串

    // 字符串转义
    let escaped = "Hello, Rust!\nThis is a new line.\tTabbed text.";
    println!("Escaped string: {}", escaped);
}

fn say_hello(s: &str) {
    println!("Hello, {}!", s);
}

fn main() {}
