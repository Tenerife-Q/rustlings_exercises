// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.
    // 报错原因: string1 的所有权在 main 函数中，而 result 的生命周期超出了 string1 的作用域。
    // 解决方法: 将 string1 的声明移动到 result 使用之前，确保 string1 在 result 使用时仍然有效。

    let string1 = String::from("long string is long");
    let result;
    
    // string2 函数作用域结束后，result 仍然引用 string1，因此不会报错。
    let string2 = String::from("xyz");
    result = longest(&string1, &string2);
    
    println!("The longest string is '{result}'");
}
