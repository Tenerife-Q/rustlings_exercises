trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

// 这里我们为 Vec<String> 实现 AppendBar trait
// append_bar 方法会在向量中添加字符串 "Bar"
// vec<String> 是一个动态数组，适合存储字符串 是一个常见的集合类型 属于标准库
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.

    // ========== 基础测试 ==========
    let mut foo = vec![String::from("Foo")].append_bar();
    println!("Vector: {:?}", foo);  // ["Foo", "Bar"]
    
    // ========== Experiment 实验 ==========
    
    // 实验1：空向量
    println!("\n=== 实验1：空向量 ===");
    let empty = Vec::<String>::new();
    let result = empty.append_bar();
    println!("空向量添加后: {:?}", result);  // ["Bar"]
    
    // 实验2：链式调用
    println!("\n=== 实验2：链式调用 ===");
    let v = vec![String::from("A")]
        .append_bar()
        .append_bar()
        .append_bar();
    println!("链式调用: {:?}", v);  // ["A", "Bar", "Bar", "Bar"]
    
    // 实验3：pop 操作顺序
    println!("\n=== 实验3：pop 操作 ===");
    let mut v = vec![String::from("First"), String::from("Second")]
        .append_bar();
    println!("原始:  {:?}", v);
    println!("pop1: {}", v.pop().unwrap());  // Bar
    println!("pop2: {}", v.pop().unwrap());  // Second
    println! ("pop3: {}", v. pop().unwrap());  // First
    
    // 实验4：结合泛型（回顾第14章）
    println!("\n=== 实验4：与泛型结合 ===");
    fn process_vec<T: AppendBar>(v: T) -> T {
        v.append_bar()
    }
    
    let v = vec![String::from("Generic")];
    let result = process_vec(v);
    println!("泛型处理: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
