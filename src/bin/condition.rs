fn main() {}

pub fn if_else() {
    let number = 3;
    if number % 4 == 0 {
        println!("Condition was true");
    } else if number % 3 == 0 {
        println!("Condition was true for number % 3");
    } else if number % 2 == 0 {
        println!("Condition was true for number % 2");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {}", number);
}
