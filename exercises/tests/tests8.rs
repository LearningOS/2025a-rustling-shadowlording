// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::time::SystemTime;

fn main() {
    // 1. 为 tests7 设置环境变量 TEST_FOO（保留原逻辑）
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 2. 为 tests8 设置编译配置（启用 feature "pass"）
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
