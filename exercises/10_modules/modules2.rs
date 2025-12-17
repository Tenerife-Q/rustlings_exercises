// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

pub mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // self 表示当前模块
    pub use self::fruits::PEAR as fruit; // 导入 fruits 模块下的 PEAR 常量，并重命名为 fruit
    pub use self::veggies::CUCUMBER as veggie; // 导入 veggies 模块下的 CUCUMBER 常量，并重命名为 veggie
    // as 关键字用于重命名导入的项，以避免命名冲突或提供更具描述性的名称。

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        // 因为在上面我们已经使用 `pub use` 将模块路径引入到 `delicious_snacks` 模块中，
        // 所以我们可以直接通过 `delicious_snacks::fruit` 和 `delicious_snacks::veggie` 来访问它们。
        // 但是不能直接使用fruit和veggie,不要delicious_snacks::,因为main函数不在delicious_snacks模块内
        
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
