// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::{Arc, Mutex};  // 新增 Mutex
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));  // 包装 Mutex
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status = status_shared.lock().unwrap();  // 获取锁
            status.jobs_completed += 1;  // 安全修改
        });
        handles.push(handle);
    }
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    // 打印最终结果
    let final_status = status.lock().unwrap();
    println!("jobs completed {}", final_status.jobs_completed);
}
