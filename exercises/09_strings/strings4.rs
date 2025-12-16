// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    
    string_slice("blue");// ""里面的内容是字符串切片类型 &str 要用 string_slice 函数

    string("red".to_string());// to_string() 方法会把 &str 转换成 String 类型 要用 string 函数

    string(String::from("hi"));// String::from()是另一种创建 String 类型的方法 要用 string 函数

    string("rust is fun!".to_owned());// to_owned() 等价于 to_string() 
    
    string("nice weather".into());// into()等价于 to_string()
    
    string(format!("Interpolation {}", "Station"));// format! 宏返回两个字符串拼接后的 String 类型

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    
    string_slice(&String::from("abc")[0..1]);// 这里的 &String::from("abc")[0..1] 是一个字符串切片类型 &str [0..1] 表示从索引0到索引1的子串
    
    string_slice(" hello there ".trim());// trim() 方法返回一个去掉前后空格的字符串切片类型 &str
    
    string("Happy Monday!".replace("Mon", "Tues"));//replace()方法作用是替换字符串中的子串，返回一个新的 String 类型
    
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());// to_lowercase() 方法返回一个所有字母都转换为小写的 String 类型
    
}
/*
总结：
1. &str 是字符串切片类型，表示对字符串的一部分或全部的引用，通常用于函数参数传递
2. String 是堆分配的可变字符串类型，拥有字符串数据的所有权，可以进行修改
3. 根据具体情况选择使用 string_slice 函数还是 string 函数

常用api参考：
- to_string()：将 &str 转换为 String 类型
- String::from()：创建一个新的 String 类型
- to_owned()：将 &str 转换为 String 类型，等价于 to_string
- into()：将 &str 转换为 String 类型，等价于 to_string()
- format!：格式化字符串宏，返回 String 类型
- trim()：去掉字符串前后空格，返回 &str 类型
- replace()：替换字符串中的子串，返回新的 String 类型
- to_lowercase()：将字符串转换为小写，返回新的 String 类型
*/
