// TODO: Fix the compiler error about calling a private function.

// pub 作用域使得模块外部的代码可以访问模块内的项。
// crate 作用域使得同一 crate 内的代码可以访问模块内的项。
// module mod 模块
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
