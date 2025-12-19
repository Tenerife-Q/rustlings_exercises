// This exercise is an altered version of the `errors4` exercise. It uses some
// concepts that we won't get to until later in the course, like `Box` and the
// `From` trait. It's not important to understand them in detail right now, but
// you can read ahead if you like. For now, think of the `Box<dyn ???>` type as
// an "I want anything that does ???" type.
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, the `Box` is declared as of type `Box<dyn Trait>` where
// `Trait` is the trait the compiler looks for on any value used in that
// context.  For this exercise, that context is the potential errors which
// can be returned in a `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError:: Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Add the correct return type `Result<(), Box<dyn ???>>`. What can we
// use to describe both errors? Is there a trait which both errors implement? 

// 错误点：缺少返回类型，函数体中使用了 ?  
// 关键词：Box<dyn Error> 是错误处理的统一方案
// 
// 含义解析：
// Box<dyn Error> 是一个装箱的动态 Error trait 对象
// - Box:  堆上分配的智能指针，用于运行时确定大小
// - dyn Error: 任何实现了 Error trait 的类型（如 ParseIntError、CreationError）
// - 好处：可以在同一个 Result 中处理多种不同的错误类型
//
// 为什么用 Box<dyn Error>？
// 1. parse() 返回 ParseIntError
// 2. new() 返回 CreationError
// 3. 两种错误都实现了 Error trait
// 4. Box<dyn Error> 可以统一表示这两种错误
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    // parse() 返回 Result<i64, ParseIntError>
    // ?  自动转换 ParseIntError 为 Box<dyn Error>
    let x: i64 = pretend_user_input.parse()?;
    
    // new() 返回 Result<PositiveNonzeroInteger, CreationError>
    // ? 自动转换 CreationError 为 Box<dyn Error>
    println!("output={:?}", PositiveNonzeroInteger:: new(x)?);
    
    // Ok(())：表示成功执行
    Ok(())
}