trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    // 提供一个默认实现 licensing_info 方法
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.

    // ========== 基础测试 ==========
    let some = SomeSoftware { version_number: 1 };
    let other = OtherSoftware {
        version_number: String::from("v2.0.0"),
    };
    
    println!("Some:  {}", some.licensing_info());
    println!("Other: {}", other.licensing_info());
    
    // ========== Experiment 实验 ==========
    
    // 实验1：覆盖默认实现
    println!("\n=== 实验1：覆盖默认实现 ===");
    
    struct CustomSoftware {
        name: String,
    }

    // 默认实现是什么
    // licensing_info 方法返回 "Default license"
    
    impl Licensed for CustomSoftware {
        fn licensing_info(&self) -> String {
            format! ("{} - MIT License", self.name)  // 覆盖默认实现
        }
    }
    
    let custom = CustomSoftware {
        name: String::from("MyApp"),
    };
    println!("Custom: {}", custom.licensing_info());
    
    // 实验2：默认实现调用其他方法（高级模式）
    println!("\n=== 实验2：默认实现调用其他方法 ===");
    
    trait AdvancedLicense {
        fn author(&self) -> String;// 需要实现的方法
        
        fn full_license(&self) -> String {
            format!("License by {}", self.author())  // 调用其他方法
        }
    }
    
    struct MyApp {
        author_name: String,
    }
    
    impl AdvancedLicense for MyApp {
        fn author(&self) -> String {
            self.author_name.clone()
        }
        // full_license 自动继承
    }
    
    let app = MyApp {
        author_name: String::from("Ferris"),
    };
    println!("Full:  {}", app.full_license());
    
    // 实验3：多个类型共享默认行为
    println!("\n=== 实验3：多个类型共享默认行为 ===");
    
    struct App1;
    struct App2;
    struct App3;
    
    impl Licensed for App1 {}
    impl Licensed for App2 {}
    impl Licensed for App3 {}
    
    let apps: Vec<Box<dyn Licensed>> = vec![ // Vec<Box<dyn Licensed>>是一个动态分发的集合 
        Box::new(App1),                      // 里面存储实现了 Licensed trait 的不同类型 这里都是空结构体
        Box::new(App2),
        Box::new(App3),
    ];

    // 打印每个应用程序的许可证信息
    // iter() 方法返回一个迭代器 
    // enumerate() 方法为每个元素添加索引
    for (i, app) in apps.iter().enumerate() {// 枚举迭代
        println!("App{}: {}", i + 1, app.licensing_info());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
