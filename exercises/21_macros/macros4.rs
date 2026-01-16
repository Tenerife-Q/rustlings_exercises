// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };// 每个分支后面加上分号 以分隔不同的分支
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };// 这里最后一个分支可加上分号 但不是必须的
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
