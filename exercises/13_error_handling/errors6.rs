// Using catch-all error types like `Box<dyn Error>` isn't recommended for
// library code where callers might want to make decisions based on the error
// content instead of printing it out or propagating it further. Here, we define
// a custom error type to make it possible for callers to decide what to do next
// when our function returns an error.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// A custom error type that we will be using in `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {// 这里添加了 from_creation 方法 用于将 CreationError 转换为 ParsePosNonzeroError
        Self::Creation(err)
    }

    // TODO: Add another error conversion function here.
    // fn from_parse_int(???) -> Self { ??? }
    fn from_parse_int(err: ParseIntError) -> Self { //(1) 这里添加了 from_parse_int 方法 用于将 ParseIntError 转换为 ParsePosNonzeroError
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: change this to return an appropriate error instead of panicking
        // when `parse()` returns an error.
        
        // 错误点：原本使用 unwrap()，会 panic；改用 map_err 和 ?  
        // let x: i64 = s.parse().unwrap();  // 错误：panic 而不是返回错误
        
        // 第一步：解析字符串为 i64
        // s.parse() 返回 Result<i64, ParseIntError>
        // map_err() 将 ParseIntError 转换为 ParsePosNonzeroError
        // ?  操作符：失败时返回 Err，成功时解包值并继续
        let x: i64 = s.parse()
            .map_err(ParsePosNonzeroError:: from_parse_int)?;
        
        // 第二步：验证 i64 是否为正非零数
        // Self::new(x) 返回 Result<Self, CreationError>
        // map_err() 将 CreationError 转换为 ParsePosNonzeroError
        // 这是最后一步，直接返回结果给调用者，没有使用 ?
        Self::new(x)
            .map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}

/*
1.
parse("not a number")
    ↓
第一步：s.parse:: <i64>()
    ↓
返回：Err(ParseIntError { ... })
    ↓
. map_err(from_parse_int)
    ↓
转换为：Err(ParsePosNonzeroError::ParseInt(... ))
    ↓
? 操作符立即返回
    ↓
最终返回：Err(ParsePosNonzeroError::ParseInt(_))

2.
parse("-555")
    ↓
第一步：s.parse:: <i64>()
    ↓
返回：Ok(-555)
    ↓
? 操作符解包 -555 继续
    ↓
第二步：Self::new(-555)
    ↓
返回：Err(CreationError::Negative)
    ↓
. map_err(from_creation)
    ↓
转换为：Err(ParsePosNonzeroError::Creation(CreationError::Negative))
    ↓
最终返回：Err(ParsePosNonzeroError::Creation(CreationError::Negative))

3.
parse("0")
    ↓
第一步：s.parse:: <i64>()
    ↓
返回：Ok(0)
    ↓
? 操作符解包 0 继续
    ↓
第二步：Self::new(0)
    ↓
返回：Err(CreationError::Zero)
    ↓
. map_err(from_creation)
    ↓
转换为：Err(ParsePosNonzeroError::Creation(CreationError::Zero))
    ↓
最终返回：Err(ParsePosNonzeroError::Creation(CreationError::Zero))

4.
parse("42")
    ↓
第一步：s.parse:: <i64>()
    ↓
返回：Ok(42)
    ↓
? 操作符解包 42 继续
    ↓
第二步：Self::new(42)
    ↓
返回：Ok(PositiveNonzeroInteger(42))
    ↓
. map_err(from_creation) 不调用 因为是 Ok
    ↓
最终返回：Ok(PositiveNonzeroInteger(42))
*/