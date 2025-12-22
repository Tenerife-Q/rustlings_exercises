// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");

    // ========== 基础测试 ==========
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s:  {}", s);  // s: FooBar
    
    // ========== Experiment 实验 ==========
    
    // 实验1：链式调用
    println!("\n=== 实验1：链式调用 ===");
    let s1 = String::from("Hello");
    let s2 = s1.append_bar().append_bar();
    println!("链式调用: {}", s2);  // HelloBarBar
    
    // 实验2：空字符串
    println!("\n=== 实验2：空字符串 ===");
    let empty = String::from("");
    let result = empty.append_bar();
    println!("空字符串: '{}'", result);  // 'Bar'
    
    // 实验3：包含特殊字符
    println!("\n=== 实验3：特殊字符 ===");
    let emoji = String::from("🦀");
    let result = emoji.append_bar();
    println!("Emoji: {}", result);  // 🦀Bar
    
    // 实验4：所有权移动（注意：self 获取所有权）
    println!("\n=== 实验4：所有权 ===");
    let s3 = String::from("Test");
    let s4 = s3.append_bar();
    // println!("{}", s3);  // ❌ 错误！s3 已被移动
    println!("只能用 s4: {}", s4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
