mod bindings;
mod condition;
mod consts;
mod vars;

mod loops;
mod ownership;
mod type_study;

fn main() {
    // immutable();
    // mt_const();
    // binding();
    // static_num();
    // if_else();
    // loops::loop_example();
    // loops::while_example();
    // loops::for_example();
    // type_study::print_int_show();
    // type_study::print_float_show();
    // type_study::nan_example();
    // type_study::boolean_example();
    // type_study::char_example();
    // type_study::sequence_example();
    // type_study::convert();
    // ownership::_ownership_example();

    // 用两种方法打印成功s1 和s2的值

    // 第一种方法
    let s1 = String::from("Hello, Rust!");
    // let s2 = ownership::take_ownership(s1.clone()); // 使用clone方法复制s1
    // 第二种方法 修改take_ownership函数
    let s2 = ownership::take_ownership(&s1); // 传递s1的引用

    //以下代码不能修改
    println!("{}", s1);
    println!("{}", s2);
}
