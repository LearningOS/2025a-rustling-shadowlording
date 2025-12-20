// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 添加注释说明空块是有意为之
    }

    let my_arr = [-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 35;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
