// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// use ???;

// 可以花括号{}来一次性引入多个项
use std::time::{SystemTime, UNIX_EPOCH};

// 也可以分开多行引入
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

// 还可以使用 as 
// use std::time::{SystemTime as ST, UNIX_EPOCH as EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        // compile error: cannot find type `SystemTime` in this scope
        // compile error: cannot find type `UNIX_EPOCH` in this scope
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
