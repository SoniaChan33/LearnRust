use std::{ffi::c_short, net::Shutdown};

fn main() {
    // immutable();
    // mt_const();
    // binding();
}

// fn immutable() {
//     let x = 5;
//     println!("The value of x is: {}", x);
// }

// fn mutable() {
//     let mut y = 10;
//     println!("The value of y is: {}", y);
//     y = 15;
//     println!("The value of y is now: {}", y);
// }

// 常量
// const  NUM: i32 = 5;
// const THREE_HOURS :i32 = 60 * 60 * 3;

// fn fmt_const() {
//     println!("The value of NUM is: {}", NUM);
//     println!("Three hours in seconds is: {}", THREE_HOURS);
//     THREE_HOURS = 100;
// }

// 静态变量
static mut NUMBER: i32 = 10;

fn static_num() {
    unsafe {
        println!("static NUMBER: {}", NUMBER += 1);
    } // 这里可以访问静态变量NUMBER unsafe代码块是必须的，因为静态变量在多线程环境下可能会引发数据竞争。
}

fn binding() {
    // 绑定生存于main函数中
    let long_lived_binding = 1;

    // 代码块， 比main函数拥有更小的作用域
    {
        // 绑定生存于代码块中
        // 这里的short_lived_binding只在这个代码块中有效
        let short_lived_binding = 2;
        println!("short_lived_binding: {}", short_lived_binding);
        // 遮蔽，在作用域中可以遮蔽成功
        let short_lived_binding = 3;
        println!(
            "short_lived_binding after shadowing: {}",
            short_lived_binding
        );

        println!("long_lived_binding: {}", long_lived_binding);
        // 遮蔽长期绑定的变量
        // 这里的long_lived_binding会遮蔽外层的同名变量
        let long_lived_binding = 4;
        println!("long_lived_binding after shadowing: {}", long_lived_binding);
    }
    println!("long_lived_binding after block: {}", long_lived_binding);
    // println!("short_lived_binding after block: {}", short_lived_binding); // 这里会报错，因为short_lived_binding在代码块外不可见
    // 遮蔽长期绑定的变量
    let long_lived_binding = 5_f32;
    println!("long_lived_binding after shadowing: {}", long_lived_binding);
}
