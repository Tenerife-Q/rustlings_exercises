#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
        // 需求：如果 value 小于 0，返回 Err(CreationError::Negative)
        // 如果 value 等于 0，返回 Err(CreationError::Zero)
        // 否则返回 Ok(PositiveNonzeroInteger(value as u64))
        if value < 0 {
            return Err(CreationError::Negative);// (1 Err(CreationError::Negative)表示创建错误为负数 ::Negative表示枚举 CreationError 的一个变体
        }
        if value == 0 {
            return Err(CreationError::Zero);// (2 Err(CreationError::Zero)表示创建错误为零 ::Zero表示枚举 CreationError 的一个变体
        }
        Ok(Self(value as u64))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
