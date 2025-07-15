use core::str;

/**
 * 结构体基本使用
 */
#[test]
pub fn struct_example() {
    // 创建结构体实例
    // 1.每个字段都需要实例化
    // 2.字段顺序可以任意
    // 3.字段名逗号分隔
    let username = String::from("someusername123");
    let user1 = User {
        active: true,
        username, // 简写
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 结构体更新语法，没有逗号
    };

    //1 访问结构体的字段
    println!("The username is {}", user1.email);

    //2 修改结构体字段（可变性）
    //      整个实例必须是可变的
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    //3 所有权
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let email = user1.email;
    // print_user(user1);
}

/**
 * 元祖结构体创建
 */
fn tuple_struct() {
    //3 使用元组结构体 tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{}", black.0);
}
/**
 * 单元结构体创建
 */
#[test]
fn unit_struct() {
    // 单元结构体创建
    let always_equal = AlwaysEqual;
}

/**
 * 结构体方法的使用
 */
#[test]
fn struct_function_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    rect2.set_width(20);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    if rect2.is_wide() {
        println!("The rectangle is wide.");
    } else {
        println!("The rectangle is not wide.");
    }

    // 使用new
    let rect3 = Rectangle::new(30, 50);

    // can_hold
    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::area(&rect3)
    );
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//单元结构体
struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        // new 方法可以创建一个 Self 的实例
        Self { width, height }
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_wide(&self) -> bool {
        self.width > self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Circle {
    radius: u32,
}
impl Circle {
    fn new(radius: u32) -> Self {
        Circle { radius }
    }
}

trait Shape {
    fn area(&self) -> u32;
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        3 * self.radius * self.radius
    }
}

/**
 * 通过impl Trait可以创建一个函数，
 * 这个函数的参数是任何实现了Shape trait的类型。
 * 这个函数可以传入圆形和矩形，都可以进行area计算，实现了通用的计算逻辑
 */
fn print_area(shape: &impl Shape) {
    println!("{}", shape.area());
}

#[test]
fn test_trait_impl() {
    // 使用trait通用计算
    let rect4 = Rectangle::new(30, 50);
    let circle1 = Circle::new(100);
    print_area(&rect4);
    print_area(&circle1);
}
use std::fmt::Display;

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.height {
            let mut s = String::new();
            for w in 0..self.width {
                s.push('6');
            }
            write!(f, "{}\n", s);
        }

        Ok(())
    }
}
use std::fmt::Debug;
impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle: width: {}, height: {}",
            self.width, self.height
        );

        Ok(())
    }
}

#[test]
fn print_the_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect1);

    let result = println!("{:?}", rect1);
}
