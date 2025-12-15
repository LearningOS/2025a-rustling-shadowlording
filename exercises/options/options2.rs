// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 修复：添加 `if let` 关键字，并声明变量 `word`
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // 修复：调整模式匹配，处理 `Vec.pop()` 返回的 `Option<Option<i8>>`
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor as i8); // 补充类型转换，避免类型不匹配
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
