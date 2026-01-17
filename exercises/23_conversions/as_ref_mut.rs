// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument
// (`.len()` returns the number of bytes in a string).
// TODO: Add the `AsRef` trait appropriately as a trait bound.

// AsRef 用于引用到引用的转换
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}
// AsRef 的好处在于，char_counter 和 byte_counter 可以接受任何实现了 AsRef<str> 的类型，比如 &str 和 String。

// Squares a number using `as_mut()`.
// TODO: Add the appropriate trait bound.

// AsMut 用于可变引用到可变引用的转换
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let num = arg.as_mut();
    *num = (*num) * (*num);
}

// 这样的好处在于，num_sq 可以接受任何实现了 AsMut<u32> 的类型的可变引用，比如 &mut u32 和 Box<u32>
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
