use std::f32::consts::E;

// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.

fn generate_nametag_text(name: String) -> Result<String, String> { // (1) 这里将 Option<String> 改为 Result<String, String>
    if name.is_empty() {
        // Empty names aren't allowed  因为 name 为空时，应该返回错误信息
        Err("Empty names aren't allowed".to_string()) // (2) 这里将 None 改为 Err(...)
    } else {
        Ok(format!("Hi! My name is {name}")) // (3) 这里将 Some(...) 改为 Ok(...)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),// as_deref() 将 Result<String, String> 转换为 Result<&str, &str>
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            //前者 generate_nametag_text().as_ref().map_err(|e| e.as_str()) 
            // 将 Result<String, String> 转换为 Result<&String, &String>，再将 &String 转换为 &str
            //后者 Err("Empty names aren't allowed") 直接是 Err(&str)
            generate_nametag_text(String::new())
                .as_ref()// as_ref() 将 Result<String, String> 转换为 Result<&String, &String>
                .map_err(|e| e.as_str()),//map_err 将 &String 转换为 &str   |e| e.as_str()是一个闭包，表示将错误值e(String)转换为&str
            Err("Empty names aren't allowed"),
            // String &str &String 三者分别代表不同的字符串类型 应用场景不同 
            // String 是拥有所有权的字符串类型，适用于需要修改或拥有字符串数据的场景
            // &str 是字符串切片，适用于只读访问字符串数据的场景
            // &String 是对 String 的引用，适用于需要传递字符串但不需要拥有它的场景
        );
    }
}
