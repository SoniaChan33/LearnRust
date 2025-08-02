pub fn binding() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("short_lived_binding: {}", short_lived_binding);
        let short_lived_binding = 3;
        println!(
            "short_lived_binding after shadowing: {}",
            short_lived_binding
        );
        println!("long_lived_binding: {}", long_lived_binding);
        let long_lived_binding = 4;
        println!("long_lived_binding after shadowing: {}", long_lived_binding);
    }
    println!("long_lived_binding after block: {}", long_lived_binding);
    let long_lived_binding = 5_f32;
    println!("long_lived_binding after shadowing: {}", long_lived_binding);
}

fn main() {
    println!("Running lesson16 - Variable Bindings");
    binding();
}
