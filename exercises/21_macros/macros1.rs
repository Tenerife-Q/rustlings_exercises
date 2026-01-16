macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    // 这里添加!是为了正确调用宏 调用宏时需要加上!
    // 宏是什么？宏是一种元编程工具 可以在编译时生成代码
    my_macro!();
}
