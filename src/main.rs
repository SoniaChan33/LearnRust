use core::{num, panic};
use std::{array, ffi::c_short, net::Shutdown};

fn main() {
    // immutable();
    // mt_const();
    // binding();
    // static_num();
    // if_else();
    // loop_example();
    // while_example();
    for_example();
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
// static mut NUMBER: i32 = 10;

// fn static_num() {
//     unsafe {
//         NUMBER += 1;
//         println!("static NUMBER: {}", NUMBER);
//     } // 这里可以访问静态变量NUMBER unsafe代码块是必须的，因为静态变量在多线程环境下可能会引发数据竞争。
// }

// fn binding() {
//     // 绑定生存于main函数中
//     let long_lived_binding = 1;

//     // 代码块， 比main函数拥有更小的作用域
//     {
//         // 绑定生存于代码块中
//         // 这里的short_lived_binding只在这个代码块中有效
//         let short_lived_binding = 2;
//         println!("short_lived_binding: {}", short_lived_binding);
//         // 遮蔽，在作用域中可以遮蔽成功
//         let short_lived_binding = 3;
//         println!(
//             "short_lived_binding after shadowing: {}",
//             short_lived_binding
//         );

//         println!("long_lived_binding: {}", long_lived_binding);
//         // 遮蔽长期绑定的变量
//         // 这里的long_lived_binding会遮蔽外层的同名变量
//         let long_lived_binding = 4;
//         println!("long_lived_binding after shadowing: {}", long_lived_binding);
//     }
//     println!("long_lived_binding after block: {}", long_lived_binding);
//     // println!("short_lived_binding after block: {}", short_lived_binding); // 这里会报错，因为short_lived_binding在代码块外不可见
//     // 遮蔽长期绑定的变量
//     let long_lived_binding = 5_f32;
//     println!("long_lived_binding after shadowing: {}", long_lived_binding);
// }

// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// // 函数无返回值的情况
// fn add_two(i: i32, j: i32) {
//     let result: i32 = i + j;
//     println!("The sum of {} and {} is: {}", i, j, result);
//     // 这里没有返回值，函数结束时会隐式返回 ()
// }

// // 函数永不返回的情况
// // ! 表示函数永不返回
// // 例如： panic! 宏会导致程序崩溃，函数不会返回
// // 这种函数通常用于错误处理或程序终止的场景。
// fn add_one(i: i32, j: i32) -> ! {
//     panic!("weeee");
// }

// // 死循环也不会返回
// fn infinite_loop() -> ! {
//     loop {
//         // 永远不会返回
//         println!("This is an infinite loop");
//     }
// }

// fn if_else() {
//     let number = 3;
//     if number % 4 == 0 {
//         println!("Condition was true");
//     } else if number % 3 == 0 {
//         println!("Condition was true for number % 3");
//     } else if number % 2 == 0 {
//         println!("Condition was true for number % 2");
//     } else {
//         println!("Condition was false");
//     }

//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is : {}", number);
// }

fn loop_example() {
    //loop 循环
    loop {
        println!("This is an infinite loop");
    }

    // break and continue
    let mut count: i32 = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("Breaking the loop at count: {}", count);
            break; // 退出循环
        }
        if count % 2 == 0 {
            println!("Skipping even count: {}", count);
            continue; // 跳过当前循环的剩余部分，继续下一次循环
        }
    }
    println!("Loop ended at count: {}", count);

    // let语句中使用loop
    let result = loop {
        count += 1;
        if count == 10 {
            println!("Breaking the loop at count: {}", count);
            break count * 2; // 返回值
        }
    };
    println!("The result of the loop is: {}", result);

    // 多层循环可以使用标签来控制跳出特定的循环层级
    // 标签可以帮助我们在多层嵌套循环中跳出特定的循环层级
    // 标签的语法是使用单引号（'）后跟一个标识符
    let mut i = 0;
    'outerloop: loop {
        loop {
            i += 1;
            println!("Inner loop iteration with i: {}", i);
            if i == 2 {
                println!("Breaking out of the inner loop at i: {}", i);
                break; // 跳出内层循环
            }
            if i == 3 {
                println!("Breaking out of the outer loop at i: {}", i);
                break 'outerloop; // 跳出外层循环
            }
        }
    }
}

fn while_example() {
    let mut count: i32 = 0;
    while count < 5 {
        count += 1;
        println!("Current count: {}", count);
    }
}

fn for_example() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for i in numbers {
        println!("Current number: {}", i)
    }

    // 直接遍历数值，逆序使用rev()
    for i in (1..=4).rev() {
        println!("Current number in range: {}", i);
    }
    // 所有权转移
    let mut vec = vec![1, 2, 3, 4, 5];
    for v in &mut vec {
        if *v == 3 {
            *v += 10; // 修改当前值
        }
        println!("Current vector value: {}", v);
    }

    // 注意：vec在这里的所有权已经转移，不能再使用vec
    println!("Vector after for loop: {:?}", vec); // 这行会报错
    // 如果需要在循环后继续使用vec，可以使用迭代器
    // for v in &mut vec {
    //     println!("Current vector value after push: {}", v);
    // }

    // 基本数据类型数组就不会转移所有权
    let arr = [1, 2, 3, 4, 5];
    for v in arr {
        println!("Current array value: {}", v);
    }
    println!("Array after for loop: {:?}", arr); // 这里可以继续使用arr，因为数组的所有权没有转移
}
