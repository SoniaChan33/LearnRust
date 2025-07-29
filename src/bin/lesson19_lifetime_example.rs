fn main() {
    let result = longest("helllshfs", "worldrus");
    println!("the longest string is {}", result);

    let str1 = "hello";
    {
        let str2 = "world";
        println!("the longest string is {}", longest(str1, str2));
    }

    test_lifetime_multiple();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn test_lifetime_multiple() {
    fn insert_value<'a, 'b>(my_vec: &'a mut Vec<&'b i32>, value: &'b i32) {
        my_vec.push(value)
    }
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;

    let a = &mut my_vec;
    insert_value(a, &val1);
    println!("a is {:?} ", a);
    let b = &mut my_vec;
    insert_value(b, &val2);
    println!("b is {:?}", b);
    println!("{my_vec:?}");
}
