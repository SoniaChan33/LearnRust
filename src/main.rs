mod bindings;
mod condition;
mod consts;
mod vars;

mod arrays_slices;
mod collection;
mod collection_practices;
mod enums_study;
mod loops;
mod option_example;
mod ownership;
mod panic_result;
mod pattern_match;
mod reference;
mod string_study;
mod struct_example;
mod type_study;

// use a::{b::log as log1, log as log2};
fn main() {
    // log1();
    // log2(); // 内部的mod需要多层调用
    a::log(); // 相对路径访问

    a::b::log(); // 绝对路径访问
}

mod a {
    const num: usize = 1;
    pub fn log() {
        println!("{}", num);
    }
    pub mod b {
        // 无论是func还是mod 在外部需要调用的话都需要加上pub
        const num2: usize = 2;
        pub fn log() {
            println!("{}", num2);
        }
    }
}
