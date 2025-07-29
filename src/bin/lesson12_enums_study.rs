#[test]
fn enum_size_analysis() {
    use std::mem::size_of;

    enum MyEnum {
        A(u8, u8),
        B,
        C {},
    }
    enum EnumA1 {
        A = 255,
    }
    enum EnumA2 {
        A = 255,
        B,
    }

    println!("MyEnum size: {} bytes", size_of::<MyEnum>());
    println!("EnumA1 size: {} bytes", size_of::<EnumA1>());
    println!("EnumA2 size: {} bytes", size_of::<EnumA2>());
}
use std::collections::btree_map::Values;

#[derive(Debug)] // 为了能够打印枚举的值
enum Pets {
    Cat(String),
    Dog { name: String, age: usize },
    Bird, // 元素类型 unit type
}

fn print_pet_info() {
    let a = Pets::Cat("Whiskers".to_string());
    let b = Pets::Dog {
        name: "Buddy".to_string(),
        age: 5,
    };

    // Debug语义
    println!("cat is {:?}", a);
    println!("dog is {:?}", b);

    //impl是用来为枚举或结构体定义方法的
    // 定义一个方法
    // 这里的 &self 表示方法是实例方法，可以访问实例的字段
    // 这里的 self 是一个不可变引用
    // 如果需要修改实例的字段，可以使用 &mut self
    impl Pets {
        fn speak(&self) {
            println!("hi");
        }
    }
    a.speak();

    // 关联函数是与类型相关的函数，可以通过类型名直接调用
    // 关联函数不需要实例化对象就可以调用
    impl Pets {
        fn log(name: String) {
            println!("Logging pet: {}", name);
        }
    }
    Pets::log("Whiskers".to_string());

    // match 语句：
    // 用于模式匹配，可以根据枚举的不同变体执行不同的代码
    match &a {
        Pets::Cat(_) => {
            println!("This is a cat");
        }
        Pets::Bird => {
            println!("This is a bird");
        }
        Pets::Dog { name, age } => {
            println!("This is a dog named {} and age {}", name, age);
        }
        // 这里的 _ 是一个通配符，表示匹配所有未被前面分支匹配的情况
        _ => {
            println!("This is not a cat");
        }
    }

    // if let 语句：
    // 用于简化模式匹配，当只关心某个特定变体时，可以使用 if let
    // 这种方式可以避免使用 match 的冗长语法
    if let Pets::Dog { name, age } = &b {
        println!("This is a dog named {} and age {}", name, age);
    } else {
        println!("This is not a dog");
    }
}

#[test]
fn option_example() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    // 使用 match 处理 Option
    match some_value {
        Some(val) => println!("Got a value: {}", val), // Some 分支
        None => println!("No value found"),            // None 分支
    }

    if let Some(val) = none_value {
        println!("Got a value: {}", val);
    } else {
        println!("No value found");
    }
}

#[test]
fn option_find_even() {
    // 使用迭代器的 find 方法查找第一个偶数
    let nums: &[i32] = &[1, 3, 5, 7, 8, 10];
    let first_even: Option<&i32> = nums.iter().find(|&&x| x % 2 == 0);
    match first_even {
        Some(&val) => println!("First even number is: {}", val),
        None => println!("No even number found"),
    }
}

#[test]
fn result_example() {
    // result 是一个枚举类型，用于表示操作的结果
    // Result<T, E> 有两个变体：Ok(T) 和 Err(E)
    // Ok(T) 表示操作成功，并包含一个值 T，Err(E) 表示操作失败，并包含一个错误值 E
    // Result<T, E> 是一个泛型枚举，可以存储任何类型的值和错误
    // Result<T, E> 常用于处理可能失败的操作，例如文件读取、网络请求等
    // Result<T, E> 的使用可以提高代码的安全性和可读性
    // Result<T, E> 的使用可以避免异常处理（exception handling）的复杂性
    let ok_value: Result<i32, &str> = Ok(42);
    let err_value: Result<i32, &str> = Err("Error occurred");

    match ok_value {
        Ok(val) => println!("Got a value: {}", val), // Ok 分支
        Err(err) => println!("Error: {}", err),      // Err 分支
    }

    if let Err(err) = err_value {
        println!("Error: {}", err);
    }
}

#[test]
fn option_result_convert() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // Option 转 Result
    let result_from_some: Result<i32, &str> = some_value.ok_or("No value found");
    let result_from_none: Result<i32, &str> = none_value.ok_or("No value found");

    match result_from_some {
        Ok(val) => println!("Got a value: {}", val),
        Err(err) => println!("Error: {}", err),
    }

    match result_from_none {
        Ok(val) => println!("Got a value: {}", val),
        Err(err) => println!("Error: {}", err),
    }

    // Result 转 Option
    let ok_value: Result<i32, &str> = Ok(42);
    let err_value: Result<i32, &str> = Err("Error occurred");
    let option_from_ok: Option<i32> = ok_value.ok();
    let option_from_err: Option<i32> = err_value.ok();
    match option_from_ok {
        Some(val) => println!("Got a value: {}", val),
        None => println!("No value found"),
    }
    match option_from_err {
        Some(val) => println!("Got a value: {}", val),
        None => println!("No value found"),
    }
}

#[test]
fn enum_api_example() {
    let option1 = Some(5);
    let option2: Option<i32> = None;

    let option_mapped = option1.map(|x: i32| x * 2); // 使用 map 方法对 Some 值进行操作
    println!("Option1 after map: {:?}", option_mapped); // 对 Some 值进行操作
    let option2_mapped = option2.map(|x: i32| x * 2); // 对 None 值进行操作时不会执行闭包
    println!("Option2 after map: {:?}", option2_mapped); // 对 None 值进行操作

    option1.and_then(|x: i32| Some(x * 2)); // and_then 方法用于链式调用
    option2.and_then(|x: i32| Some(x * 2)); // 对 None 值进行操作时不会执行闭包
    let option1_or_else = option1.or_else(|| Some(0)); // or_else 方法用于提供默认值
    println!("Option1 or else: {:?}", option1_or_else); // 对 Some 值提供默认值
    let option2_or_else = option2.or_else(|| Some(10)); // 对 None 值提供默认值
    println!("Option2 or else: {:?}", option2_or_else); // 对 None 值提供默认值
    option1.unwrap_or(0); // unwrap_or 方法用于获取 Some 值或提供默认值
    option2.unwrap_or(0); // 对 None 值提供默认值
    option1.is_some(); // 检查是否为 Some 值
    option2.is_some(); // 检查是否为 Some 值
    option1.is_none(); // 检查是否为 None 值

    let result: Result<i32, i32> = Ok(1);
    let error: Result<(), &str> = Err("An error occurred");
    result.map(|_| println!("Success!")); // 使用 map 方法对 Ok 值进行操作
    error.map(|_| println!("Success!")); // 对 Err 值进行操作时
    // 不会执行闭包
    result.and_then(|val| Ok(val)); // and_then 方法用于链式调用 返回的是result类型
    error.and_then(|val| Ok(val)); // 对 Err 值进行操作时不会执行闭包
    // ⚠️ or_else只会在 Err 分支执行
    result.or_else(|val: i32| Err(val)); // or_else 方法用于提供默认值
    error.or_else(|_| Err(())); // 对 Err 值提供默认值
    result.unwrap_or_else(|val: i32| 1); // unwrap_or_else 返回的是最终的值
    error.unwrap_or_else(|val| {
        println!("Error occurred: {}", val);
        ()
    }); // 对 Err 值提供默认值
    result.is_ok(); // 检查是否为 Ok 值
    error.is_ok(); // 检查是否为 Ok 值
    result.is_err(); // 检查是否为 Err 值
    error.is_err(); // 检查是否为 Err 值
}

#[test]
fn enum_practice() {
    // 分析enum所占内存大小
    enum MyEnum {
        A(u8, u8),
        B,
        C {},
    }
    // 计算 MyEnum 的大小
    let size = std::mem::size_of::<MyEnum>();
    println!("Size of MyEnum: {}", size);
}

fn main() {}
