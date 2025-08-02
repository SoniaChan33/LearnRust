use core::num;

// fn iter_study() {
//     let v1 = vec![1, 2, 3];
//     let v1_iter = v1.iter();
//     for var in v1_iter {
//         println!("{}", var);
//     }

//     let mut v1_iter2 = v1.iter();
//     while let Some(var) = v1_iter2.next() {
//         println!("{}", var);
//     }

//     // 使用trait
//     let counter = Counter {};
// }

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// struct Counter {}

// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(1)
//     }
// }
use std::iter::Iterator;
// 定义斐波那契迭代器结构体，存储当前和下一个数值
struct Fibonacci {
    a: u64,
    b: u64,
}

// 实现 Iterator trait（核心）
impl Iterator for Fibonacci {
    type Item = u64; // 明确关联类型为 u64

    // 实现 next 方法，严格遵循 fn next(&mut self) -> Option<Self::Item> 签名
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        // 计算下一组值（斐波那契规则：b = 前 a + 前 b；a = 前 b）
        self.a = self.b;
        self.b = current + self.b;
        Some(current) // 无限迭代，始终返回 Some
    }
}

// 初始化方法：从 0, 1 开始
impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

// 测试：输出前 10 个斐波那契数
fn main() {
    let fib = Fibonacci::new();
    // take(10) 是迭代器适配器，证明 Fibonacci 被识别为迭代器
    for (i, num) in fib.take(10).enumerate() {
        println!("第 {} 个: {}", i + 1, num);
    }
}
