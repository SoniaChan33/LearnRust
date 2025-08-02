pub fn array_example() {
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);

    // 默认初始值
    let arr1: [i32; 4] = [10; 4];
    println!("Array with default value: {:?}", arr1);

    // 数组长度
    println!("Array length: {}", arr1.len());

    // 数组遍历
    for index in 0..arr.len() {
        // 遍历索引
        println!("Element at index {}: {}", index, arr[index]);
    }
    for element in arr1.iter() {
        // 使用迭代器遍历
        println!("Element: {}", element)
    }
    arr1.iter().for_each(|x: &i32| println!("Element: {}", x)); // 使用闭包遍历

    // 可变数组
    let mut arr2 = [1, 2, 3, 4, 5];
    arr2[0] = 10; // 修改第一个元素
    println!("Modified array: {:?}", arr2);

    // 数组值传递
    println!("Before update: {:?}", arr2);
    update(arr2);
    println!("After update: {:?}", arr2); // 注意这里的 arr2 仍然是原来的值，因为数组是值传递

    // 数组引用传递
    println!("Before update_mut: {:?}", arr2);
    update_mut(&mut arr2);
    println!("After update_mut: {:?}", arr2); // 这里的 arr2 已经被修改，因为传递的是可变引用
}

fn update(mut arr: [i32; 5]) {
    println!("Updating array: {:?}", arr);
    for index in 0..arr.len() {
        arr[index] = 100; // 修改每个元素
    }
    println!("Updated array inside function: {:?}", arr);
}

fn update_mut(arr: &mut [i32; 5]) {
    println!("Updating array with mutable reference: {:?}", arr);
    for index in 0..arr.len() {
        arr[index] = 100; // 修改每个元素
    }
    println!("Updated array with mutable reference: {:?}", arr);
}

fn slice_example() {
    let arr = [1, 2, 3, 4, 5];
    let slice1: &[i32] = &arr[1..3]; // 包含索引1和2的元素
    println!("Slice from index 1 to 3: {:?}", slice1);

    let slice2: &[i32] = &arr[..3]; // 包含前3个元素
    println!("Slice of first three elements: {:?}", slice2);

    let slice3: &[i32] = &arr[2..]; // 从索引2开始到末尾
    println!("Slice from index 2 to end: {:?}", slice3);

    let arr2: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b: &[i32] = &[];
    println!("Slice of empty array: {:?}", b.is_empty());

    for i in arr.windows(3) {
        // 遍历数组的窗口
        println!("Window of size 3: {:?}", i);
    }
}

fn product(num: &[i32]) -> i32 {
    let mut result = 1;
    for &i in num {
        result *= i;
    }
    result
}

#[test]
fn feature() {
    array_example();
}

#[test]
fn feature_slice() {
    slice_example();
}

#[test]
fn feature_array_practice() {
    let nums: [i32; 4] = [1, 2, 3, 4];
    let answer: Vec<i32> = arr_practice(&nums[..]);
    println!("Result of arr_practice: {:?}", answer);
}

fn arr_practice(nums: &[i32]) -> Vec<i32> {
    let n: usize = nums.len();
    let mut answer = vec![1; n]; // 初始化为全1

    for i in 1..n {
        answer[i] = answer[i - 1] * nums[i - 1];
    }
    let mut suffix = 1;
    for i in (0..n).rev() {
        answer[i] *= suffix;
        suffix *= nums[i];
    }

    answer
}

fn main() {}
