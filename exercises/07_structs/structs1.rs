struct ColorRegularStruct {
    // 常规结构体
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors? u8最合适
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8); // 元组结构体

#[derive(Debug)]
struct UnitStruct;// 单元结构体 无字段

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =

        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =


        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =

        let unit_struct = UnitStruct;

        
        let message = format!("{unit_struct:?}s are fun!");
        //测试里assert_eq!(message, "UnitStructs are fun!");
        //能通过，是因为UnitStruct的Debug输出默认是结构体名（首字母大写），所以{unit_struct:?}变成"UnitStruct"，加上s就是"UnitStructs"啦！
        assert_eq!(message, "UnitStructs are fun!");
    }
}

/*
包含字段的结构体有三种类型：
1. 常规结构体（Regular Structs）：包含命名字段的结构体，类似于其他编程语言中的类或结构体。
2. 元组结构体（Tuple Structs）：包含未命名字段的结构体，类似于元组，但有自己的类型
3. 单元结构体（Unit Structs）：不包含任何字段的结构体，类似于空元组 ()，通常用于标记或实现特定的行为。

每种结构体类型都有其特定的用途和适用场景：
- 常规结构体适用于需要表示具有多个属性的复杂数据类型的情况。
- 元组结构体适用于需要简单封装多个值但不需要命名字段的情况。
- 单元结构体适用于需要表示某种状态或行为但不需要存储数据的情况。


*/