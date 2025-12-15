// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Clippy提示：`is_none()`可简化为`matches!`或直接判断，但此处保留逻辑即可
    }

    let my_arr = [-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    value_a.swap(&mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
