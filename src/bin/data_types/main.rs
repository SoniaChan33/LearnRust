pub fn _print_int_show() {
    let _integer: i32 = 2147483647; // 最大值
    let _integer: i32 = -2147483648; // 最小值
    let _integer: i32 = 0x1F; // 十六进制
    let _integer: i32 = 0o17; // 八进制
    let _integer: i32 = 0b1111_0000; // 二进制
    let _integer: i32 = 1_000_000; // 使用下划线分隔数字
    let _integer: u8 = b'A'; // 字符转换为整数
}
pub fn _print_float_show() {
    let _float: f64 = 3.14;
    let _float: f64 = 1.0;

    // assert_eq!(0.1 + 0.2, 0.3); // 浮点数相加
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3 = {:x}", abc.2.to_bits());
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3 = {:x}", xyz.2.to_bits());

    assert_eq!(abc.0 + abc.1, abc.2); // 浮点数相加
    assert_eq!(xyz.0 + xyz.1, xyz.2); // 浮点数相加
}

pub fn _nan_example() {
    let nan_value: f64 = (-1.1_f64).sqrt(); // 计算负数的平方根
    assert!(nan_value.is_nan()); // 检查是否为NaN
    println!("NaN: {}", nan_value);
}

pub fn _boolean_example() {
    let t = true;
    let f: bool = false;
    println!("true: {}, false: {}", t, f);
}

pub fn _char_example() {
    let c: char = 'A';
    let emoji: char = '😊'; // unicode字符也可以作为Rust的字符
    println!("char: {}, emoji: {}", c, emoji);
}

pub fn _sequence_example() {
    let seq = 1..=5; // 包含5的范围
    for i in seq {
        println!("Sequence: {}", i);
    }
}

pub fn convert() {
    let a: i32 = 10;
    let b: f64 = a as f64; // 将i32转换为f64
    println!("Converted value: {}", b);

    let c: f64 = 3.14;
    let d: i32 = c as i32; // 将f64转换为i32
    println!("Converted value: {}", d);
}

fn main() {}