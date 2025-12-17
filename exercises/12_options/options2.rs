fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.

        // if let 语法 用于匹配枚举类型中的某个变体，并解构其内部数据
        // 如果没有if let, 会报错：expected enum `Option`, found `&str`
        // 因为 optional_target 是 Option 类型，而 target 是 &str 类型
        // 所以需要使用 if let 来解构 optional_target 中的值
        if let Some(value) = optional_target {
            assert_eq!(value, target);
        } else {
            panic!("The option was None!");
        }
    }

    #[test]
    fn layered_option() {

        // 初始化数据 NONE + Some(1..=10)
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        // while let 语法 用于在循环中匹配枚举类型中的某个变体，并解构其内部数据
        // Vec::pop() 方法返回一个 Option 类型的值
        // 如果 Vec 为空，则返回 None
        // 如果 Vec 不为空，则返回 Some(元素值)
        // 这里Some(Some(value)) 表示 Vec 中的元素是 Some(value) 类型是 Option<i8>
        // pop() 返回的值是 Some(Some(value)) 或 None 类型是 Option<Option<i8>>

        while let Some(Some(value)) = optional_integers.pop() {
            assert_eq!(value, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}


// option 拆盒
// if let / while let / match
