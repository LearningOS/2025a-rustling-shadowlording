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
        // 场景1：编译通过 + 测试通过（默认推荐）
        // assert_eq!(1 + 1, 2);
        
        // 场景2：编译通过 + 测试失败（验证错误场景）
        assert_eq!(1 + 1, 3);
    }
}
