use crossbeam_channel::bounded;
use std::fs;
use std::thread;
use std::time::Duration;

// ----------------实现多线程文件处理器----------------------
// 同时可以处理4个文件读取，说明有四个接受端 ，一个发送端
fn process_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("Processing file: {}", filename);
            println!("Content: {}", content);
        }
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error);
        }
    }
}

// --------------------使用channel实现程序的优雅停止------------------

// 使用channel实现程序的优雅停止
// 任务类型：包含任务ID和模拟耗时
type Task = Option<u32>; // None作为停止信号

// 处理任务
fn process_task(task_id: u32) {
    println!("处理任务{}...", task_id);
    thread::sleep(Duration::from_secs(1)); // 模拟耗时1秒
    println!("任务{}处理完成", task_id);
}

fn main() {
    let (tx, rx) = bounded(4);
    let num_threads = 4;
    let file_paths = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
        "file5.txt",
        "file6.txt",
        "file7.txt",
        "file8.txt",
        "file9.txt",
        "file10.txt",
    ];

    let mut handles = vec![];
    // 启动 4 个工作线程
    for i in 0..num_threads {
        let rx: crossbeam_channel::Receiver<String> = rx.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            for path in rx {
                let content = fs::read_to_string(path).unwrap_or_else(|e| format!("Error: {}", e));
                println!("Worker {}: {}", i, content);
            }
        });
        handles.push(handle);
    }

    // 发送文件路径
    for path in file_paths {
        tx.send(path.to_string()).unwrap();
    }
    drop(tx);

    for h in handles {
        h.join().unwrap()
    }

    //-------------------------------

    let (tx, rx) = bounded(4);
    let num_workers = 3;
    let mut handles = vec![];

    // 创建工作线程
    for i in 0..num_workers {
        let rx = rx.clone();
        let handle = thread::spawn(move || {
            println!("工作线程{}启动", i);
            // 循环接收任务，直到收到None（停止信号）
            while let Ok(task) = rx.recv() {
                match task {
                    Some(task_id) => process_task(task_id),
                    None => {
                        println!("工作线程{}收到停止信号，退出", i);
                        break; // 退出循环，线程结束
                    }
                }
            }
        });
        handles.push(handle);
    }

    // 主线程发送任务
    for task_id in 1..=5 {
        tx.send(Some(task_id)).unwrap();
    }

    // 发送停止信号（每个线程一个）
    for _ in 0..num_workers {
        tx.send(None).unwrap();
    }

    // 等待所有工作线程退出
    for handle in handles {
        handle.join().unwrap();
    }
    println!("所有线程已优雅退出");
}
