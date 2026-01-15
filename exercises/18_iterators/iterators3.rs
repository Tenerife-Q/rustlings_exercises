#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

// 1. 实现除法逻辑
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    // 特殊情况：i64::MIN / -1 会导致溢出，因为正数范围比负数范围少 1
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }
    Ok(a / b)
}

// 2. 目标：将 Result 的序列转为 包含序列的 Result
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    // collect 能够将 Iterator<Result<T, E>> 转换为 Result<Vec<T>, E>
    // 只要其中有一个 Err，整个结果就是 Err
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

// 3. 目标：简单的 Result 列表
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    // 这里 collect 只是简单地把 map 后的结果收集到 Vec 里
    // into_iter() 将数组转换为迭代器
    // map() 对每个元素应用 divide 函数
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    // 可以在这里打印测试
    println!("{:?}", result_with_list());
    println!("{:?}", list_of_results());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
