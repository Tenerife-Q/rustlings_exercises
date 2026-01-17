// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (mut string, command) in input {
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(times) => {
                    for _ in 0..times {
                        string.push_str("bar");
                    }
                    output.push(string);
                }
            }
        }

        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

// // 1. 按逗号分割字符串
//         let parts: Vec<&str> = s.split(',').collect();

//         // 2. 检查字段数量（必须是 2）
//         if parts.len() != 2 {
//             return Err(ParsePersonError::BadLen);
//         }

//         // 3. 获取姓名，4. 检查是否为空
//         let name = parts[0];
//         if name.is_empty() {
//             return Err(ParsePersonError::NoName);
//         }

//         // 5. & 6. 解析年龄，如果失败，将原生的 ParseIntError 包装进我们的错误类型
//         let age = parts[1]
//             .parse::<u8>()
//             .map_err(ParsePersonError::ParseInt)?;

//         Ok(Person {
//             name: name.to_string(),
//             age,
//         })
//     }
//}