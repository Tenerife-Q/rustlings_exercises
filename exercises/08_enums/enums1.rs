#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Quit,
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
    Resize{width: i32, height: i32},
}

fn main() {
    // {:?} 作用是用于调试输出枚举的变体和数据
    println!("{:?}", Message::Resize{width: 10, height: 20});
    println!("{:?}", Message::Move{x: 5, y: 15});
    println!("{:?}", Message::Echo(String::from("Hello, Rust!")));
    println!("{:?}", Message::ChangeColor(255, 0, 0));
    println!("{:?}", Message::Quit);
}
