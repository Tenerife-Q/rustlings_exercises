// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.



/*
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    // let shared_numbers = ???;
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // let child_numbers = ???;
        let child_numbers = Arc::clone(&shared_numbers);
        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
} */


#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    // 1. 原始数据：一个包含 0-99 的数组。
    let numbers: Vec<_> = (0..100u32).collect();

    // 2. 包装数据：
    // 我们不能直接把 `numbers` 分给 8 个线程，因为 Rust 的所有权规则规定：
    // 一个变量同时只能有一个主人。如果给了线程 A，线程 B 就没份了。
    // 所以我们用 `Arc`（原子引用计数）把它包装起来。
    // 此时，引用计数 = 1。
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // 3. 复制“通行证”：
        // Arc::clone 并不复制外面的 Vec 数据，它只是把“引用计数”加 1。
        // 每一个 child_numbers 都是一个指向相同内存地址的“智能指针”。
        // 循环 8 次，就会产生 8 个独立的指针。
        let child_numbers = Arc::clone(&shared_numbers);

        // 4. 移交所有权：
        // `move` 关键字把上面那个克隆出来的 `child_numbers` 指针移交给新线程。
        // 每个线程现在都拥有自己的一份“通行证”。
        let handle = thread::spawn(move || {
            // 5. 使用数据：
            // 虽然每个线程都有 child_numbers，但它们指向的是堆上同一块 numbers 数据。
            let sum: u32 = child_numbers
                .iter()
                .filter(|&&n| n % 8 == offset)
                .sum();
            println!("Sum of offset {offset} is {sum}");
            
            // 线程结束时，child_numbers 会被销毁，引用计数自动减 1。
        });

        join_handles.push(handle);
    }

    // 6. 等待收工：
    // 主线程等待所有子线程计算完毕。
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
    
    // 当 main 函数结束，shared_numbers 被销毁，引用计数归零，内存正式释放。
}