use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    
    // 解决方法：利用 mpsc::Sender 实现了 Clone 的特性
    // 在进入第一个线程前，克隆一份 tx
    let tx1 = tx.clone();
    
    // 为了让两个线程都能访问 q 的不同部分，我们需要先解构 q，
    // 否则整个 q 会在第一个线程创建时被 move 进去。
    let first_vals = q.first_half;
    let second_vals = q.second_half;

    thread::spawn(move || {
        for val in first_vals {
            println!("Sending {val:?}");
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in second_vals {
            println!("Sending {val:?}");
            // 这里直接使用原始的 tx，它被 move 进了第二个线程
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
