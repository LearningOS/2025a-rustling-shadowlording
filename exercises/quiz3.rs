// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
// 定义 Grade 特征：所有可作为成绩的类型需实现此特征
pub trait Grade {
    fn format_grade(&self) -> String;
}

// 为 f32 实现 Grade 特征（数值型成绩）
impl Grade for f32 {
    fn format_grade(&self) -> String {
        self.to_string()
    }
}

// 为 &str 实现 Grade 特征（字母型成绩，如 "A+"）
impl Grade for &str {
    fn format_grade(&self) -> String {
        self.to_string()
    }
}

// 为 String 实现 Grade 特征（字母型成绩的所有权版本）
impl Grade for String {
    fn format_grade(&self) -> String {
        self.clone()
    }
}

// 使用泛型参数 G 约束成绩类型（必须实现 Grade 特征）
pub struct ReportCard<G: Grade> {
    pub grade: G,
    pub student_name: String,
    pub student_age: u8,
}

impl<G: Grade> ReportCard<G> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.grade.format_grade() // 调用特征方法格式化成绩
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1, // 数值型成绩（f32）
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+", // 字母型成绩（&str）
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
