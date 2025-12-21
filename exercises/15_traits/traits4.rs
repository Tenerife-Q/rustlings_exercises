trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
    
// 这里我们为 compare_license_types 函数添加泛型约束
// 使其接受实现了 Licensed trait 的任意类型
// 这样我们就可以比较不同类型的软件的许可证信息
// 函数返回一个布尔值 表示两个软件的许可证信息是否相同
// 泛型约束使用了 trait bound 语法 T: Licensed 和 U: Licensed
// 这样可以确保传入的参数都实现了 Licensed trait
// 函数体内调用了 licensing_info 方法 来获取许可证信息 并进行比较
fn compare_license_types<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
