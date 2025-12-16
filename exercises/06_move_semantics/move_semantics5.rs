#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

/*
fn main() {
    // 注意：要使用可变引用，变量本身必须是 mut 的
    let mut s = String::from("Hello");

    // 模式 1: 只读借用 (Read)
    read_only(&s);   // s 依然可用

    // 模式 2: 可变借用 (Modify/Push)  <-- 这是你提到的 push 场景
    modify(&mut s);  // s 被修改了，但依然归 main 所有，依然可用
    
    // 模式 3: 所有权转移 (Consume)
    consume(s);      // s 被移走，彻底不可用
    // println!("{}", s); // ❌ 报错：value borrowed here after move
}

// 对应 get_char
fn read_only(data: &String) {
    println!("只读不改: {}", data.len());
}

// 对应 vec.push()
fn modify(data: &mut String) {
    data.push_str(" World"); // 直接在原内存地址上追加
    println!("原地修改: {}", data);
}

// 对应 string_uppercase
fn consume(data: String) {
    println!("吃掉数据: {}", data);
    // 函数结束，data 在这里被 drop，内存释放
}
*/