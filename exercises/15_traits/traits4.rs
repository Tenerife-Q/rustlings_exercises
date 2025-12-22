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

    // ========== 基础测试 ==========
    let same = compare_license_types(SomeSoftware, OtherSoftware);
    println!("Same license: {}", same);  // true
    
    // ========== Experiment 实验 ==========
    
    // 实验1：不同的实现
    println!("\n=== 实验1：不同的实现 ===");
    
    struct FreeSoftware;
    struct ProprietarySoftware;
    
    impl Licensed for FreeSoftware {
        fn licensing_info(&self) -> String {
            "GPL v3".to_string()
        }
    }
    
    impl Licensed for ProprietarySoftware {
        fn licensing_info(&self) -> String {
            "Proprietary".to_string()
        }
    }
    
    let different = compare_license_types(FreeSoftware, ProprietarySoftware);
    println!("不同许可证: {}", different);  // false
    
    // 实验2：impl Trait 语法
    println!("\n=== 实验2：impl Trait 语法 ===");
    
    fn simple_compare(s1: impl Licensed, s2: impl Licensed) -> bool {
        s1.licensing_info() == s2.licensing_info()
    }
    
    let result = simple_compare(SomeSoftware, OtherSoftware);
    println!("简化语法: {}", result);
    
    // 实验3：where 子句（更清晰）
    println!("\n=== 实验3：where 子句 ===");
    
    // 这里我们使用 where 子句 来定义泛型约束 也可以直接在函数签名中使用
    fn detailed_compare<T, U>(s1: T, s2: U) -> String
    where
        T: Licensed,
        U: Licensed,
    {
        format!(
            "Software 1: {}\nSoftware 2: {}\nSame: {}",
            s1.licensing_info(),
            s2.licensing_info(),
            s1.licensing_info() == s2.licensing_info()
        )
    }
    
    println!("{}", detailed_compare(FreeSoftware, ProprietarySoftware));
    
    // 实验4：结合错误处理（第13章）
    println!("\n=== 实验4：结合错误处理 ===");
    
    fn validate_license<T: Licensed>(software: T) -> Result<String, String> {
        let info = software.licensing_info();
        if info.contains("Default") {
            Err(String::from("Must specify a license"))
        } else {
            Ok(info)
        }
    }
    
    match validate_license(FreeSoftware) {
        Ok(license) => println!(" Valid:  {}", license),
        Err(e) => println!(" Error: {}", e),
    }
    
    match validate_license(SomeSoftware) {
        Ok(license) => println!(" Valid: {}", license),
        Err(e) => println!(" Error: {}", e),
    }
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
