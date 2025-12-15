// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 两个相等的可比较值，断言成功，测试通过
        assert_eq!(42, 42);
        // 也可以用其他类型（如字符串、布尔值），只要满足
        // assert_eq!("hello", "hello");
        // assert_eq!(true, !false);
    }
}