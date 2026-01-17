// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     // TODO: Fix the Clippy lint.  
//     for x in option {
//         res += x;
//     }

//     println!("{res}");
// }

fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用 if let 替代 for 循环
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

// 函数式编程风格
// fn main() {
//     let mut res = 42;
//     let option = Some(12);

//     // 将 option 中的值加到 res 上，如果没有值则加 0
//     res += option.unwrap_or(0);

//     println!("{res}");
// }