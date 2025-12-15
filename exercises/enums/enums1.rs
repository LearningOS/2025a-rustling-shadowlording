#[derive(Debug)]
enum Message {
    Quit,          
    Echo(String),  
    Move { x: i32, y: i32 },  
    ChangeColor(u8, u8, u8),  // 修复：去掉多余的斜杠
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello")));  // 补充String参数
    println!("{:?}", Message::Move { x: 10, y: 20 });        // 补充结构体字段
    println!("{:?}", Message::ChangeColor(255, 0, 0));       // 补充RGB参数
}
