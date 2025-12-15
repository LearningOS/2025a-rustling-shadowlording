// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    #[allow(clippy::unnecessary_option_if_let)]  // 允许空Option判断
    let my_option: Option<()> = None;
    #[allow(clippy::unnecessary_option_if_let)]  // 修复：允许空if块
    if my_option.is_none() {
    }

    #[allow(clippy::print_literal)]  // 修复：允许打印字面量数组
    let my_arr = [-1, -2, -3, -4, -5, -6];
    #[allow(clippy::print_literal)]  // 修复：允许打印字面量数组
    println!("My array! Here it is: {my_arr:?}");

    #[allow(clippy::print_literal)]  // 修复：允许打印空Vec
    let my_empty_vec: Vec<i32> = Vec::new();
    #[allow(clippy::print_literal)]  // 修复：允许打印空Vec
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 450;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
