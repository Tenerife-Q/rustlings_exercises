// struct Point {
//     x: u64,
//     y: u64,
// }

// enum Message {
//     Resize { width: u64, height: u64 },
//     Move(Point),
//     Echo(String),
//     ChangeColor(u8, u8, u8),
//     Quit,
// }

// struct State {
//     width: u64,
//     height: u64,
//     position: Point,
//     message: String,
//     // RGB color composed of red, green and blue.
//     color: (u8, u8, u8),
//     quit: bool,
// }

// impl State {
//     // 这里定义了State结构体的方法，用于处理不同的Message变体
//     // 每个方法中第一个参数是&mut self，表示这些方法会修改State实例的状态
//     // 这是语法糖，等同于fn method_name(state: &mut State, other_params...)
//     fn resize(&mut self, width: u64, height: u64) {
//         self.width = width;
//         self.height = height;
//     }

//     fn move_position(&mut self, point: Point) {
//         self.position = point;
//     }

//     fn echo(&mut self, s: String) {
//         self.message = s;
//     }

//     fn change_color(&mut self, red: u8, green: u8, blue: u8) {
//         self.color = (red, green, blue);
//     }

//     fn quit(&mut self) {
//         self.quit = true;
//     }

//     // 这里 fn process(state: &mut State, message: Message) 
//     fn process(&mut self, message: Message) {
//         // TODO: Create a match expression to process the different message
//         // variants using the methods defined above.
//         match message {
//             Message::Resize { width, height } => self.resize(width, height),
//             Message::Move(point) => self.move_position(point),
//             Message::Echo(s) => self.echo(s),
//             Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
//             Message::Quit => self.quit(),
//         }
//     }
// }

// fn main() {
//     // You can optionally experiment here.
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_match_message_call() {
//         let mut state = State {
//             width: 0,
//             height: 0,
//             position: Point { x: 0, y: 0 },
//             message: String::from("hello world"),
//             color: (0, 0, 0),
//             quit: false,
//         };

//         state.process(Message::Resize {
//             width: 10,
//             height: 30,
//         });
//         state.process(Message::Move(Point { x: 10, y: 15 }));
//         state.process(Message::Echo(String::from("Hello world!")));
//         state.process(Message::ChangeColor(255, 0, 255));
//         state.process(Message::Quit);

//         assert_eq!(state.width, 10);
//         assert_eq!(state.height, 30);
//         assert_eq!(state.position.x, 10);
//         assert_eq!(state.position.y, 15);
//         assert_eq!(state.message, "Hello world!");
//         assert_eq!(state.color, (255, 0, 255));
//         assert!(state.quit);
//     }
// }






// ==========================================
// 1. 定义阶段：造盒子（Define Data Structures）
// ==========================================

// Point: 这是一个普通的结构体 (Classic Struct)
// 用途：作为一个小零件，用来存储坐标。
#[derive(Debug)] // 这是一个属性，为了让 println! {:?} 能打印出它
struct Point {
    x: u64,
    y: u64,
}

// Message: 这是一个枚举 (Enum) —— 核心概念：带数据的标签
// 它的每个成员（Variant）形状都不一样，就像不同形状的快递包裹。
enum Message {
    // 形状1：像结构体一样，有两个命名字段
    Resize { width: u64, height: u64 },
    
    // 形状2：像元组一样，包裹着另一个类型 (Point结构体)
    // 这叫 "New Type" 模式的一种变体
    Move(Point),
    
    // 形状3：包裹着一个 String 字符串
    Echo(String),
    
    // 形状4：包裹着三个 u8 数字 (元组)
    ChangeColor(u8, u8, u8),
    
    // 形状5：什么都不带，就是一个纯信号
    Quit,
}

// State: 状态机的主体
// 用途：存储程序运行时的所有数据。
#[derive(Debug)] // 方便打印调试
struct State {
    width: u64,
    height: u64,
    position: Point, // 结构体嵌套结构体
    message: String,
    // 这是一个元组类型 (Tuple)，存储 RGB 颜色
    color: (u8, u8, u8),
    quit: bool,
}

// ==========================================
// 2. 实现阶段：定义行为 (Define Behavior)
// ==========================================

impl State {
    // -------------------------------------------------------
    // 辅助方法 (Helper Methods)
    // -------------------------------------------------------
    // 参数解析：&mut self
    // 1. self: 指代调用这个方法的 State 实例本身。
    // 2. mut:  表示我有权修改这个实例内部的数据。
    // 3. &:    表示我是"借用" (Borrow)，用完就还，不拿走所有权。
    
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // -------------------------------------------------------
    // 核心逻辑：分发器 (Dispatcher)
    // -------------------------------------------------------
    // 参数解析：
    // &mut self:      我要修改 State 的状态。
    // message: Message: 这里没有 &，是按值传递 (Pass by Value/Move)。
    //                 意味着 Message 被传入函数后，会被消费掉（用完即焚）。
    //                 这是合理的，因为指令执行一次就没用了。
    fn process(&mut self, message: Message) {
        
        // Match 模式匹配：拆包现场
        // 这里的逻辑是：把 message 包裹拆开，根据里面的东西，决定调用哪个辅助方法。
        // 穷举所有可能的 Message 变体 (Variant)，也就是每种场景都要处理。不然会报错 error: non-exhaustive patterns: `...` not covered
        // 枚举类型需要通过  枚举名::变体名  来引用并作为一个带标签的容器 里面装着要处理的数据
        match message {
            // 场景 1: 解构命名字段
            // 把 width 字段的值拿出来给临时变量 w，height 给 h 但这里没有临时变量h而是直接用height
            Message::Resize { width, height } => {
                println!("处理 Resize: 宽 {}, 高 {}", width, height);
                self.resize(width, height)
            },

            // 场景 2: 解构元组封装 (嵌套结构体)
            // 把包裹里的 Point 拿出来，命名为 point
            Message::Move(point) => {
                println!("处理 Move: 移动到 ({}, {})", point.x, point.y);
                self.move_position(point)
            },

            // 场景 3: 解构 String
            Message::Echo(s) => {
                println!("处理 Echo: 消息内容 '{}'", s);
                self.echo(s)
            },

            // 场景 4: 解构多个值的元组
            // 必须按顺序给这三个值起名字 (r, g, b)
            Message::ChangeColor(r, g, b) => {
                println!("处理 ChangeColor: R={}, G={}, B={}", r, g, b);
                self.change_color(r, g, b)
            },

            // 场景 5: 空变体
            // 不需要解构任何数据
            Message::Quit => {
                println!("处理 Quit: 退出程序");
                self.quit()
            },
        }
    }
}


/*
具体 Message::XXX 变体如何被创建和传递给 process 方法的过程：
// 1. 创建消息
let resize_msg = Message::Resize { width: 800, height: 600 };

// 2. process方法接收消息
state.process(resize_msg);

// 3. 在process中匹配
match resize_msg {
    Message::Resize { width, height } => {
        // width = 800, height = 600
        self.resize(800, 600); // 更新状态
    }
    _ => {} // 其他变体
}
*/


// ==========================================
// 3. 运行阶段：Main 函数实验区
// ==========================================

fn main() {
    // 1. 初始化 State (必须是 mut，因为后面要改它)
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("init"),
        color: (0, 0, 0),
        quit: false,
    };

    println!("初始状态: {:?}", state);
    println!("---------------------------------");

    // 2. 模拟发送指令
    // 注意：每次调用 process，都创建了一个新的 Message 实例并移交给函数
    // Message初始化时间是在语句Message::XXX处
    // 然后被process函数的参数message接收，message的作用域在process函数体内，函数体结束后message离开作用域被销毁

    // 发送 Resize
    state.process(Message::Resize { width: 800, height: 600 });
    
    // 发送 Move (注意 Point 也是现造现传)
    state.process(Message::Move(Point { x: 50, y: 100 }));
    
    // 发送 Echo
    state.process(Message::Echo(String::from("Hello Rust!")));
    
    // 发送 ChangeColor
    state.process(Message::ChangeColor(255, 128, 0));
    
    // 发送 Quit
    state.process(Message::Quit);

    println!("---------------------------------");
    println!("最终状态: {:?}", state);
}

// ==========================================
// 4. 测试模块：单元测试
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}