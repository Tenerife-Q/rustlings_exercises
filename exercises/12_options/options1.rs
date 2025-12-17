// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    if hour_of_day > 23 {
        None
    } else if hour_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let ice_creams = maybe_ice_cream(12).unwrap();
        assert_eq!(ice_creams, 5); // Don't change this line.

        //如果第一句不改 可以改第二句 assert_eq!(maybe_ice_cream(12).unwrap(), 5); 使类型匹配
        let ice_creams = maybe_ice_cream(12);
        assert_eq!(ice_creams.unwrap(), 5);


        // 如果不想用 unwrap 的话，也可以这样做：
        if let Some(ice_creams) = maybe_ice_cream(12) {
            assert_eq!(ice_creams, 5); // Don't change this line.
        } else {
            panic!("There should be some ice cream!");
        }

        // Another way to do it:
        match maybe_ice_cream(12) {
            Some(value) => assert_eq!(value, 5), // Don't change this line.
            None => panic!("There should be some ice cream!"),
        }
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
