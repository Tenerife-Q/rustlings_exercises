// 换顺序 先定义宏 再调用宏
// 函数和宏对比
// 函数在运行时被调用 而宏在编译时展开
// 宏可以生成代码 代码可以是函数 结构体 枚举等
// 宏可以接受不同数量和类型的参数
// 宏可以用来简化重复代码
// 宏的定义必须在调用之前

// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}