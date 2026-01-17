// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
/* 
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    if my_option.is_some() {
        println!("{}", my_option.unwrap());
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    value_a = value_b;
    value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}
*/
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    
    // 1. 使用 if let 替代 is_some() + unwrap()
    // Clippy: unnecessary_unwrap
    if let Some(value) = my_option {
        println!("{}", value);
    }

    // 2. 修复数组中丢失的逗号
    // Clippy: possible_missing_comma
    // 原代码中 -3 -4 被计算成了 -7，因为中间缺少逗号
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    // 3. 修复 Vec 初始化
    // Clippy: unit_hash (或者相关 lint)
    // resize() 返回的是单元类型 ()，而不是 Vec 本身。
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // 或者 my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 4. 使用 std::mem::swap 替代手动交换
    // Clippy: manual_swap
    // 原代码逻辑是错误的（a=b; b=a 会导致两者都等于旧的 b）
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}