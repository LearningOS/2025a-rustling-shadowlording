// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
fn main() {
    let option = Some(12);  // 修复：变量声明需在使用前
    let res = 420 + option.unwrap_or(0);
    println!("{}", res);
}
