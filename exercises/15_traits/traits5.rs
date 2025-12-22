trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.

// 这里我们为 some_func 函数添加泛型约束
// 使其接受同时实现了 SomeTrait 和 OtherTrait 的任意类型
// 函数返回一个布尔值，表示调用 some_function 和 other_function 的结果的逻辑与
// 泛型约束使用了 trait bound 语法 T: SomeTrait + OtherTrait
// 这样可以确保传入的参数都实现了这两个 trait
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.

    // ========== 基础测试 ==========
    println!("SomeStruct: {}", some_func(SomeStruct));
    println!("OtherStruct: {}", some_func(OtherStruct));
    
    // ========== Experiment 实验 ==========
    
    // 实验1：三个 trait bounds
    println!("\n=== 实验1：三个 trait bounds ===");
    
    trait ThirdTrait {
        fn third_function(&self) -> bool {
            true
        }
    }
    
    impl ThirdTrait for SomeStruct {}
    
    // trait bounds 是一种约束 泛型参数必须实现指定的 trait
    // 具体实现
    fn complex_func<T>(item: T) -> bool
    where
        T: SomeTrait + OtherTrait + ThirdTrait,  // 三个 traits
    {
        item.some_function() && item.other_function() && item.third_function()
    }
    
    println!("复杂函数: {}", complex_func(SomeStruct));
    
    // 实验2：结合泛型和 Display
    println!("\n=== 实验2：结合 Display ===");
    
    use std::fmt::Display;
    
    struct DisplayableStruct {
        value: i32,
    }
    
    impl Display for DisplayableStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Value: {}", self.value)
        }
    }
    
    impl SomeTrait for DisplayableStruct {}
    impl OtherTrait for DisplayableStruct {}
    
    fn print_and_check<T>(item: T) -> bool
    where
        T: SomeTrait + OtherTrait + Display,
    {
        println!("{}", item);
        item.some_function() && item.other_function()
    }
    
    let d = DisplayableStruct { value:  42 };
    println!("结果: {}", print_and_check(d));
    
    // 实验3：impl Trait 语法
    println!("\n=== 实验3：impl Trait 语法 ===");
    
    // 使用复杂语法
    fn complex_syntax<T>(item: T) -> bool
    where
        T: SomeTrait + OtherTrait,
    {
        item.some_function() && item.other_function()
    }

    // 使用简化语法
    fn simple_syntax(item: impl SomeTrait + OtherTrait) -> bool {
        item.some_function() && item.other_function()
    }
    
    println!("简化语法: {}", simple_syntax(SomeStruct));
    
    // 实验4：返回 impl Trait
    println!("\n=== 实验4：返回 impl Trait ===");
    
    fn create_struct() -> impl SomeTrait + OtherTrait {
        SomeStruct
    }
    
    let s = create_struct();
    println!("返回的结构体: {}", s.some_function());
    
    // 实验5：结合 Vec（第9章集合）
    println!("\n=== 实验5：结合 Vec ===");
    
    fn check_all<T:  SomeTrait + OtherTrait>(items: Vec<T>) -> bool {
        items. iter().all(|item| {
            item.some_function() && item.other_function()
        })
    }
    
    let items = vec![SomeStruct, SomeStruct, SomeStruct];
    println! ("所有都通过: {}", check_all(items));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
