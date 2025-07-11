pub fn immutable() {
    let x = 5;
    println!("The value of x is: {}", x);
}

pub fn mutable() {
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is now: {}", y);
}
