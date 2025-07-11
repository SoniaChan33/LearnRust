pub fn _ownership_example() {
    let s1: String = String::from("Hello");
    let s2: String = s1; // 转移所有权
    // println!("{}", s1); // 此处会报错，因为s1的所有权已转移
    drop(s2); // rust中可以手动释放资源,但通常不需要这样做,因为Rust会在变量超出作用域时自动释放资源
    // println!("{}", s2);
}

pub fn take_ownership(s: &String) -> String {
    println!("Taking ownership of: {}", s);
    s.clone() // 返回s的克隆
}
