// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. 检查输入是否为空
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 2. 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 3. 检查分割后是否恰好2个部分
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 4. 提取姓名并检查是否为空
        let name = parts[0].trim();
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 5. 提取年龄并解析为usize
        let age = parts[1].trim().parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        // 6. 返回成功结果
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}
