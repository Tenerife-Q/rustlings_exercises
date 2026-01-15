fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly) 不显式使用return
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables 不适用额外变量
    // For an extra challenge, don't use:
    // - recursion 不使用递归
    (1..=num).product()
    // 1..=num 创建一个从1到num的范围（包含num）
    // product() 方法计算范围内所有数字的乘积 是迭代器中一个常用方法
    // 为什么它能处理 0! = 1？ 因为在 Rust 中，空乘积的结果正好定义为 1，返回乘法单位元
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
