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

// use serde_json::{Value, json};
// #[test]
// fn match_practice() {
//     #[derive(Debug)]
//     struct Address {
//         street: String,
//         city: String,
//     }

//     #[derive(Debug)]
//     struct Info {
//         name: String,
//         age: u32,
//         email: String,
//         address: Address,
//         phone_numbers: Vec<String>,
//     }

//     fn parse_json(json: &str) -> Info {
//         let v: Value = serde_json::from_str(json).unwrap();
//         Info {
//             name: v["name"].as_str().unwrap().to_string(),
//             age: v["age"].as_u64().unwrap() as u32,
//             email: v["email"].as_str().unwrap().to_string(),
//             address: Address {
//                 street: v["address"]["street"].as_str().unwrap().to_string(),
//                 city: v["address"]["city"].as_str().unwrap().to_string(),
//             },
//             phone_numbers: v["phone_numbers"]
//                 .as_array()
//                 .unwrap()
//                 .iter()
//                 .map(|x| x.as_str().unwrap().to_string())
//                 .collect(),
//         }
//     }
//     let json_str = r#"
//         {
//         "name": "Alice",
//         "age": 30,
//         "email": "alice@example.com",
//         "address": {
//             "street": "123 Main St",
//             "city": "Wonderland"
//         },
//         "phone_numbers": ["123-456-7890", "987-654-3210"]
//         }
//         "#;
//     let info = parse_json(json_str);
//     println!("{:?}", info);
// }

// use serde_json::{Result, Value};
// use std::fmt;

// // 定义错误类型（替代简单的 String 错误）
// #[derive(Debug)]
// enum ParseError {
//     Json(serde_json::Error),
//     FieldMissing(&'static str),
//     TypeMismatch(&'static str, &'static str), // 字段名 + 期望类型
//     NumberRange(&'static str),                // 数字超出范围
// }

// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ParseError::Json(e) => write!(f, "JSON 解析错误: {e}"),
//             ParseError::FieldMissing(field) => write!(f, "字段缺失: {field}"),
//             ParseError::TypeMismatch(field, expect) => {
//                 write!(f, "字段 {field} 类型错误，期望 {expect}")
//             }
//             ParseError::NumberRange(field) => write!(f, "字段 {field} 数值超出范围"),
//         }
//     }
// }

// impl From<serde_json::Error> for ParseError {
//     fn from(e: serde_json::Error) -> Self {
//         ParseError::Json(e)
//     }
// }

// #[derive(Debug)]
// struct Address {
//     street: String,
//     city: String,
// }

// #[derive(Debug)]
// struct Info {
//     name: String,
//     age: u32,
//     email: String,
//     address: Address,
//     phone_numbers: Vec<String>,
// }

// // 解析函数：使用模式匹配提取字段，返回 Result 处理错误
// fn parse_json(json: &str) -> Result<Info, ParseError> {
//     let value: Value = serde_json::from_str(json)?;

//     // 1. 提取 name（字符串类型）
//     let name = match value.get("name") {
//         Some(Value::String(s)) => s.to_string(),
//         Some(_) => return Err(ParseError::TypeMismatch("name", "字符串")),
//         None => return Err(ParseError::FieldMissing("name")),
//     };

//     // 2. 提取 age（非负整数，且在 u32 范围内）
//     let age = match value.get("age") {
//         Some(Value::Number(n)) => n
//             .as_u64()
//             .ok_or(ParseError::TypeMismatch("age", "非负整数"))?
//             .try_into()
//             .map_err(|_| ParseError::NumberRange("age"))?,
//         Some(_) => return Err(ParseError::TypeMismatch("age", "数字")),
//         None => return Err(ParseError::FieldMissing("age")),
//     };

//     // 3. 提取 email（字符串类型）
//     let email = match value.get("email") {
//         Some(Value::String(s)) => s.to_string(),
//         Some(_) => return Err(ParseError::TypeMismatch("email", "字符串")),
//         None => return Err(ParseError::FieldMissing("email")),
//     };

//     // 4. 提取嵌套对象 address
//     let address = match value.get("address") {
//         Some(Value::Object(obj)) => {
//             let street = match obj.get("street") {
//                 Some(Value::String(s)) => s.to_string(),
//                 Some(_) => return Err(ParseError::TypeMismatch("address.street", "字符串")),
//                 None => return Err(ParseError::FieldMissing("address.street")),
//             };
//             let city = match obj.get("city") {
//                 Some(Value::String(s)) => s.to_string(),
//                 Some(_) => return Err(ParseError::TypeMismatch("address.city", "字符串")),
//                 None => return Err(ParseError::FieldMissing("address.city")),
//             };
//             Address { street, city }
//         }
//         Some(_) => return Err(ParseError::TypeMismatch("address", "对象")),
//         None => return Err(ParseError::FieldMissing("address")),
//     };

//     // 5. 提取数组 phone_numbers（元素为字符串）
//     let phone_numbers = match value.get("phone_numbers") {
//         Some(Value::Array(arr)) => arr
//             .iter()
//             .map(|elem| match elem {
//                 Value::String(s) => Ok(s.to_string()),
//                 _ => Err(ParseError::TypeMismatch("phone_numbers 元素", "字符串")),
//             })
//             .collect::<Result<Vec<_>, _>>()?,
//         Some(_) => return Err(ParseError::TypeMismatch("phone_numbers", "数组")),
//         None => return Err(ParseError::FieldMissing("phone_numbers")),
//     };

//     Ok(Info {
//         name,
//         age,
//         email,
//         address,
//         phone_numbers,
//     })
// }

// fn main() {
//     let json_str = r#"
//     {
//         "name": "Alice",
//         "age": 30,
//         "email": "alice@example.com",
//         "address": {
//             "street": "123 Main St",
//             "city": "Wonderland"
//         },
//         "phone_numbers": ["123-456-7890", "987-654-3210"]
//     }
//     "#;

//     match parse_json(json_str) {
//         Ok(info) => println!("解析结果:\n{:#?}", info),
//         Err(e) => eprintln!("解析失败: {e}"),
//     }
// }
