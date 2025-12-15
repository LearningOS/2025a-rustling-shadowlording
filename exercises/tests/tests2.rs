// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        // 示例1：错误的数学等式
        assert!(1 + 1 == 3); 
        // 示例2：错误的字符串判断
        assert!("rust".contains("go"));
    }
}
