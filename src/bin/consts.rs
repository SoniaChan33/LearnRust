pub const NUM: i32 = 5;
pub const THREE_HOURS: i32 = 60 * 60 * 3;

pub fn fmt_const() {
    println!("The value of NUM is: {}", NUM);
    println!("Three hours in seconds is: {}", THREE_HOURS);
    // THREE_HOURS = 100; // 常量不可修改
}

static NUMBER: i32 = 10;

pub fn static_num() {
    unsafe {
        println!("static NUMBER: {}", NUMBER);
    }
}

fn main() {}
