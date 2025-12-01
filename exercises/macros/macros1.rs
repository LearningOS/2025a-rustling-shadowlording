// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

macro_rules! my_macro {
    ($name:expr) => {  
        println!("Hello, {}! Check out my macro!", $name);
    };
}

fn main() {
    my_macro!("Rust");  
}