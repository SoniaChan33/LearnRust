use std::hash::DefaultHasher;
use std::hash::Hash;
use std::vec;
#[test]
fn vec_example() {
    // 创建一个空的vec
    let mut v: Vec<i32> = Vec::new();
    // 使用宏来创建一个veck k
    let mut v1: Vec<i32> = vec![1, 2, 3];

    // 添加元素
    v.push(5);

    // 访问元素
    // 1.使用索引
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // 2.使用get方法
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 修改元素
    v[0] = 4;

    // 迭代元素
    for i in &v {
        println!("{}", i);
    }

    // 进阶的用法
    // 1.使用枚举来存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 2.容量与重新分配
    let mut v = Vec::with_capacity(10);
    v.push(1);
    println!("{}", v.capacity());

    // 常见的错误
    // 不安全的索引访问
    let v2 = vec![1, 2, 3, 4, 5];
    // println!("{}", v2[100]); // 运行时会出错

    // 可变引用和不可变引用混用
    let mut v3 = vec![1, 2, 3, 4, 5];
    {
        let first: &i32 = &v3[0];
        println!("{}", first);
    }
    v3.push(6);
}

use std::collections::HashMap;

use serde_json::map;
#[test]
fn hash_map_example() {
    // 基本操作
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 获取元素
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    match score {
        Some(score) => println!("{}", score),
        None => println!("None"),
    }

    // 遍历
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // 更新
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Blue")).or_insert(50);

    // 合并两个集合
    let mut map1 = HashMap::new();
    map1.insert(1, "one");
    let mut map2 = HashMap::new();
    map2.insert(2, "two");
    map2.insert(3, "three");
    for (k, v) in &map2 {
        map1.entry(*k).or_insert(&v);
    }
    println!("{:?}", map1);
}

fn hashmap_ownership() {
    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    let team_score = 10;
    scores.insert(team_name.clone(), team_score.clone());
    println!("{:?}", scores);

    println!("{}", team_name);
}

/**
 * 通过vec实现stack
 */
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 初始化空栈
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 入栈：向尾部添加元素
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 出栈：移除并返回尾部元素（空栈时返回 None）
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 查看栈顶：返回尾部元素的引用（空栈时返回 None）
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}
#[test]
// 测试用例
fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("栈顶元素（peek）: {:?}", stack.peek()); // 输出: Some(3)
    println!("出栈元素（pop）: {:?}", stack.pop()); // 输出: Some(3)
    println!("出栈后栈顶: {:?}", stack.peek()); // 输出: Some(2)
}

/**
 * 通过hashmap计算单词频率
 */

fn count_frequency(s: &str) -> HashMap<&str, i32> {
    let mut letters: HashMap<&str, i32> = HashMap::new();
    for ch in s.split_whitespace() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    return letters;
}

#[test]
fn test_count_frequency() {
    let s = "hello world hello rust";
    let letters = count_frequency(s);
    println!("{:?}", letters);
}
