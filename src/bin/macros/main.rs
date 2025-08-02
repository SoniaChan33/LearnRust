fn main() {}

#[test]
fn macro_test() {
    say_hello!();
    assert_eq!(repeat!("x", 3), "xxx");
    assert_eq!(sum!(1, 2, 3, 4, 5), 15);
    assert_eq!(max_value!(1, 8, 9), 9);
}

#[macro_export]
macro_rules! say_hello {
    () => {
        println!("hello!");
    };
    ($name:expr) => {
        println!("hello {}!", $name);
    };
}

#[macro_export]
macro_rules! repeat {
    () => {
        println!("repeat!");
    };
    ($s:expr, $n:expr) => {
        std::string::String::from($s).repeat($n)
    };
}

#[macro_export]
macro_rules! sum {
    () => {
        println!("sum!");
    };
    ($($x:expr),*) => {
        {
            let mut total = 0;
            $(total += $x;)*
            total
        }
    };
}

#[macro_export]
macro_rules! max_value {
    () => {
        println!("max_value!");
    };
    ($x:expr, $y:expr, $z:expr) => {
        if $x > $y {
            if $x > $z { $x } else { $z }
        } else {
            if $y > $z { $y } else { $z }
        }
    };
}
