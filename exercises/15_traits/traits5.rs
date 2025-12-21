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
