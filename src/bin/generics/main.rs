use std::{io::Result, iter};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = lagrest(&number_list);
    println!("the largest number is {}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // 创建一个可以存储10个i32的缓冲区
    let buffer: ArrayBuffer<i32, 10> = ArrayBuffer::new();
}

fn lagrest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 定义一个固定大小的数组类型
struct ArrayBuffer<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T, const N: usize> ArrayBuffer<T, N>
where
    T: Default + Copy,
{
    fn new() -> Self {
        ArrayBuffer {
            data: [Default::default(); N],
            len: 0,
        }
    }
}
