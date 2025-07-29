use std::hash::DefaultHasher;
use std::hash::Hash;
use std::vec;

fn main() {}

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