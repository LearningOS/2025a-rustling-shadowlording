// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
ffn main() {
    let option = Some(12);
    let res = 42 + option.unwrap_or(0); 
    println!("{}", res);
}
