// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main() -> Result<(), ParseIntError> {  // (1) 这里将 main 函数的返回类型改为 Result<(), ParseIntError>
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?; // 使用 ? 运算符传播错误

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
    Ok(())//  (2) 这里添加 Ok(()) 作为函数的返回值 返回成功状态
    //没有返回Err分支 因为ParseIntError 是在 total_cost 函数中通过 ? 运算符传播的
}
