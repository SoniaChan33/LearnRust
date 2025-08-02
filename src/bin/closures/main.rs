fn main() {
    let double = |x| x * 2;
    println!("{}", apply_closure(double));
    closure_example();
}

fn closure_example() {
    // 闭包可以捕获其所在域中的变量
    let x: i32 = 4;
    let equal_to_x = |y| y == x;
    let y = 4;
    assert!(equal_to_x(y));

    // move关键字转移所有权
    let x = vec![1, 2, 3];
    let equal_to_Y = move |y| y == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_Y(y));
}

// 闭包例子
fn apply_closure<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(2)
}
