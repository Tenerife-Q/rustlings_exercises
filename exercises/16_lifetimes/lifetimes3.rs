// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
// 为结构体添加生命周期注解
// 结构体 Book 中的字段 author 和 title 都是字符串切片引用(&str)，
// 需要为结构体添加生命周期注解，确保引用在结构体实例存在期间是有效的。

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
