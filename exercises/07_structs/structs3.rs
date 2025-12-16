// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Package` struct, and we want to test some logic attached to it.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }

        //这里 使用 Self 来构造 Package 实例 因为impl块内的Self就是当前结构体类型
        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Add the correct return type to the function signature.
    fn is_international(&self) -> bool {
        // TODO: Read the tests that use this method to find out when a package
        // is considered international.
        // 业务逻辑是判断是否为国际快递

        self.sender_country != self.recipient_country
    }

    // TODO: Add the correct return type to the function signature.
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: Calculate the package's fees.
        // 业务逻辑是根据重量和每克费用计算总费用

        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    // You can optionally experiment here.
    // 创建Package实例一
    let package = Package::new(
        String::from("China"),
        String::from("USA"),
        2000
    );
    
    println!("国际包裹？{}", package.is_international());
    println!("2分/克的运费：{}分", package.get_fees(2));


    // 创建Package实例二
    let package2 = Package::new(
        String::from("Germany"),
        String::from("Germany"),
        500
    );
    println!("国际包裹？{}", package2.is_international());
    println!("3分/克的运费：{}分", package2.get_fees(3));

    // 创建Package实例三（会触发panic）
    // let package3 = Package::new(
    //     String::from("France"),
    //     String::from("Italy"),
    //     5
    // );
}

#[cfg(test)]
mod tests {
    use super::*;  // 导入当前模块的所有公共项（包括Package结构体和方法）

    // 测试用例1：验证重量不足会触发panic
    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");
        Package::new(sender_country, recipient_country, 5); // 重量5克（<10）
    }

    // 测试用例2：验证国际包裹判断
    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");
        let package = Package::new(sender_country, recipient_country, 1200);
        assert!(package.is_international()); // 发件国≠收件国 → 应为true
    }

    // 测试用例3：验证本地包裹判断
    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone(); // 克隆相同字符串
        let package = Package::new(sender_country, recipient_country, 1200);
        assert!(!package.is_international()); // 发件国=收件国 → 应为false
    }

    // 测试用例4：验证运费计算
    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");
        let cents_per_gram = 3;
        let package = Package::new(sender_country, recipient_country, 1500);
        assert_eq!(package.get_fees(cents_per_gram), 4500); // 1500*3=4500
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000); // 1500*6=9000
    }
}