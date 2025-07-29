use std::{boxed, collections::linked_list, f32::consts::TAU, ops::Deref, time::Instant, vec};

// 递归数据结构
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 类型擦除
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// drop 语义
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
const ARRAY_SIZE: usize = 100000;

/**
 * 创建一个大型数组并将其分配在堆上，然后测量和比较分配在堆和栈上的性能差异
 */
#[test]
fn boxed_practice() {
    // 测量在栈分配的时间
    let stack_start = Instant::now();
    let big_array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let stack_end = stack_start.elapsed();

    // 测量在堆分配的时间
    let stack_start: Instant = Instant::now();
    let big_boxed_array: Box<[i32]> = vec![0; ARRAY_SIZE].into_boxed_slice();
    let stack_end = stack_start.elapsed();

    println!("stack time:{:?}", stack_start);
    println!("stack end:{:?}", stack_end);
}

/**
 * 创建一个包含1_000_000个元素的数据，分别将其分配在堆和栈上。使用std::time::Instant来测量分配和访问时间。
 */
#[test]
fn feature() {}

fn main() {
    println!("Running new_lesson.rs");

    // 创建一个Box,将整数分配到堆上
    let boxed_int = Box::new(5);
    println!("Boxed int: {}", boxed_int);

    // 创建一个Box,将字符串分配到堆上
    let boxed_string = Box::new(String::from("hello, world"));
    println!("Boxed string: {}", boxed_string);

    // 允许处理动态大小类型，比如切片和向量
    let slice = &[1, 2, 3];
    let boxed_slice: Box<&[i32; 3]> = Box::new(slice);
    println!("Boxed slice: {:?}", boxed_slice);

    // 允许处理动态大小类型，比如结构体和元组
    let boxed_tuple = Box::new((String::from("hello"), 5));
    println!("Boxed tuple: {:?}", boxed_tuple);

    // 创建一个Box,将结构体分配到堆上
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }
    let boxed_person = Box::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    println!("Boxed person: {:?}", boxed_person);

    // 递归数据结构
    let list = List::Cons(1, Box::new(List::Nil));
    let list1 = List::Cons(2, Box::new(list));
    println!("List: {:?}", list1);

    //类型擦除
    // let shape_list = vec![
    //     Circle { radius: 5.0 },
    //     Rectangle {
    //         width: 10.0,
    //         height: 5.0,
    //     },
    // ]; // 这样会报错，因为List中的元素类型不同
    let shape_list: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 5.0,
        }),
    ];

    // 自动使用drop trait
    let c: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my Drop1"),
    };
    let d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my Drop2"),
    };
    println!();
}

/**
 * ---------------文件系统----------------------------
 */
// 节点枚举 用来区分文件夹还是文件，文件夹通过box包装来避免递归类型的大小歧义

enum Node {
    File(String),
    Folder(Box<FolderNode>),
}

// 文件夹节点：文件夹名，文件夹内容（Node的集合）
struct FolderNode {
    name: String,
    contents: Vec<Node>,
}

// 文件节点： 文件名，文件内容
struct FileNode {
    name: String,
    contents: String,
}

trait FileSystem {
    fn create_file(&mut self, name: String) -> Result<(), String>;
    // 创建文件夹并且返回文件夹节点引用
    fn create_folder(&mut self, name: String) -> &mut FolderNode;
    fn list_contents(&self);
}

impl FileSystem for FolderNode {
    // 1.创建文件
    fn create_file(&mut self, name: String) -> Result<(), String> {
        self.contents.push(Node::File(name));
        Ok(())
    }
    // 2.创建文件夹
    fn create_folder(&mut self, name: String) -> &mut FolderNode {
        // 新建文件夹节点
        let new_folder: FolderNode = FolderNode {
            name: name,
            contents: Vec::new(),
        };
        self.contents.push(Node::Folder(Box::new(new_folder)));

        match self.contents.last_mut() {
            Some(Node::Folder(folder)) => folder,
            _ => panic!("Invalid operation"),
        }
    }
    // 3.列出内容
    fn list_contents(&self) {
        // 打印出当前文件夹的名称
        println!("{}", self.name);
        // 遍历子节点
        for node in &self.contents {
            match node {
                Node::Folder(folder) => {
                    // 如果是文件夹，则调用文件夹的list_contents方法
                    folder.list_contents();
                }
                Node::File(file) => {
                    // 如果是文件，则打印出文件名
                    println!("{}", file);
                }
            }
        }
    }
}

#[test]
fn test_file_system() {
    // 1. 创建根文件夹
    let mut root = FolderNode {
        name: "root".to_string(),
        contents: Vec::new(),
    };

    // 2.创建文件和子文件夹
    root.create_file("file1.txt".to_string());
    let mut sub1 = root.create_folder("subfolder1".to_string());
    sub1.create_file("file2.txt".to_string());
    // 3. 子文件嵌套操作
    let mut sub2 = sub1.create_folder("subfolder2".to_string());
    let mut sub3 = sub2.create_folder("subfolder3".to_string());
    sub3.create_file("sub_file.txt".to_string());

    // 4. 遍历文件系统
    println!("File System:");
    root.list_contents();
}
