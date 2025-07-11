pub fn loop_example() {
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
    let result: i32 = loop {
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

pub fn while_example() {
    let mut count: i32 = 0;
    while count < 5 {
        count += 1;
        println!("Current count: {}", count);
    }
}

pub fn for_example() {
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

    let a = [4, 3, 2, 1];

    // 遍历索引和值
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}
