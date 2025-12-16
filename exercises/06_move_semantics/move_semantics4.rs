fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}

/*
 *  规则：在同一作用域内，针对同一块数据，同一时间只能存在一个可变引用 (&mut)。
 * 
 *  let y = &mut x; -> y 借走了 x 的写权限。

    y.push(42); -> 这是 y 最后一次出现。编译器判定 y 的借用在此刻立即终止。 rust新版NLL编译器的借用终止时机更加智能化，
    不再局限于变量的作用域结束，而是根据最后一次使用的位置来判定借用的结束时间。

    let z = &mut x; -> 此时 x 已经是自由身，z 可以安全地借走写权限。
 */