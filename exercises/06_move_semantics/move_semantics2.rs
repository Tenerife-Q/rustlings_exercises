fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}



fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // Clone vec0 to create a new vector with the same data 没有所有权转移 只是深拷贝 但是性能会差一些
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}


/*
另一种做法
// 1. 函数参数改为 &Vec<i32> (借用/只读引用)
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // 2. 在函数内部进行 clone，创建一个可变的新变量
    let mut new_vec = vec.clone(); 
    new_vec.push(88);
    new_vec
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        // 3. 传参时传入引用 &vec0
        let vec1 = fill_vec(&vec0); 

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
} 
*/
