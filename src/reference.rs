pub fn reference_example() {
    // 创建一个可变字符串s1
    let mut s: String = String::from("Hello");
    // 创建s1的不可变引用
    let s1: &String = &s;
    println!("s1: {}", s1);

    // 创建一个可变引用
    let s2: &mut String = &mut s;
    println!("s2: {}", s2);
}

pub fn slice_example() {
    // 数组引用
    let a = [1, 2, 3];
    let b = &a[0..1];
    let c = &a[0..=1];
    // 字符串引用
    let s = String::from("Hello, Rust!");
    let slice1: &str = &s[0..5]; // "Hello"
    let slice2: &str = &s[7..]; // "Rust!"
    let slice3: &str = &s[..]; // "Hello, Rust!"
}

// 演示悬垂指针的例子
pub fn dangling_pointer_example() {
    let a: String = get_a();

    fn get_a() -> String {
        "a".to_string()
    }
}

// 生命周期
pub fn lifetime_example() {
    let large = longest("a", "ab");
    println!("The longest string is: {}", large);

    // 生命周期注解示例
    // 这里的 'a 表示返回的引用与参数 x 和 y 的生命周期
    // 相同，确保返回的引用在调用者的作用域内有效
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

#[test]
fn test_lifetime_example() {
    lifetime_example();
}

#[test]
fn test_dangling_pointer_example() {
    dangling_pointer_example();
}

#[test]
fn test_reference_example() {
    reference_example();
}

#[test]
fn test_slice_example() {
    slice_example();
}
