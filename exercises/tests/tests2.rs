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
    fn you_can_assert_eq() {
        // 修复1：assert_eq!需要传入两个待比较的参数（解决编译错误）
        // 修复2：传入不相等的值（让测试失败，满足题目要求）
        assert_eq!(1 + 1, 3);
    }
}
