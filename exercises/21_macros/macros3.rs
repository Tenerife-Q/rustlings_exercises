// TODO: Fix the compiler error without taking the macro definition out of this
// module.
// 你需要做的是为宏添加 #[macro_export] 属性
// 这样宏就可以在模块外部使用了
// 宏导出后 可以在 main 函数中直接调用宏
// 当然这是在模块外部调用的情况 如果在同一个模块内调用 则不需要导出
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
