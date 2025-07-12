pub fn _ownership_example() {
    let s1: String = String::from("Hello");
    let s2: String = s1; // 转移所有权
    // println!("{}", s1); // 此处会报错，因为s1的所有权已转移
    drop(s2); // rust中可以手动释放资源,但通常不需要这样做,因为Rust会在变量超出作用域时自动释放资源
    // println!("{}", s2);
}

pub fn take_ownership(s: &String) -> &String {
    println!("Taking ownership of: {}", s);
    &s // 返回s的引用
}

#[test]
fn test_take_ownership() {
    let s1 = String::from("Hello, Rust!");
    let s2 = take_ownership(&s1); // 传递s1的
    // 引用而不是所有权
    assert_eq!(s1, *s2); // 确保s1和s2的值相同
    println!("s1: {}, s2: {}", s1, s2);
}
