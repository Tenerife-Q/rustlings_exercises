// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
// <<是位运算左移，相当于乘以2的n次方。
// 1 << 3 = 0001 << 3 = 1000 = 8 相当于二进制移位
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
    }
}
