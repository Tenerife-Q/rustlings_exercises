# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)


## 为类型实现 Traits
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为 NewsArticle 实现 Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self. location)
    }
}

// 为 Tweet 实现 Summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:  {}", self.username, self. content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 1.75 is out!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Rust Wins Programming Language of the Year"),
        location: String::from("Global"),
        author: String::from("Tech News"),
        content: String:: from("Rust continues to dominate... "),
    };

    println! ("Tweet: {}", tweet.summarize());
    println!("Article: {}", article.summarize());
}

# 实现规则
// ✅ 可以：我们的 trait + 我们的类型
impl Summary for NewsArticle { }

// ✅ 可以：标准库的 trait + 我们的类型
impl Display for NewsArticle { }

// ❌ 不可以：标准库的 trait + 标准库的类型
impl Display for Vec<String> { }  // 编译错误！


## 默认实现

# 基础默认实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // 默认实现
    }
}

// 使用默认实现
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is awesome"),
        location: String::from("Internet"),
        author: String::from("Ferris"),
        content: String::from("... "),
    };

    println! ("{}", article.summarize());  // 输出：(Read more...)
}

# 覆盖默认实现
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format! ("{}:  {}", self.username, self. content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String:: from("ferris"),
        content: String:: from("🦀"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());  // 输出：ferris: 🦀
}

# 默认实现调用其他方法
pub trait Summary {
    // 必须实现的方法
    fn summarize_author(&self) -> String;

    // 默认实现，调用了 summarize_author
    fn summarize(&self) -> String {
        format!("(Read more from {}... )", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self. username)
    }
    // summarize 使用默认实现
}

fn main() {
    let tweet = Tweet {
        username:  String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet. summarize());
    // 输出：(Read more from @horse_ebooks...)
}