#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // 提供对 optional_point 的匹配
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }
    
    // 或者可以这样写
    match optional_point {
        Some(ref point) => println!("{point:?}"),
        None => println!("No point found"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}

//ref 关键字用于在模式匹配中创建对值的引用，而不是获取值的所有权
//在 match 语句中使用 ref，可以避免移动值，从而允许在后续代码中继续使用该值
//在这个例子中，使用 ref p 创建了对 Point 结构体的引用
//如果不使用 ref，匹配到 Some(p) 会将 Point 的所有权移动到 p 变量中
//这意味着在 match 语句之外，optional_point 将不能再被使用，因为它的所有权已经被移动了