use crossbeam_channel as channel;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// ------------------实现多线程任务调度器------------------------
/**
 * 多线程任务调度器，能够接收多个任务，并将多个任务分配给多个线程执行。需要使用send和sync trait
 * 来确保任务调度器可以安全地被多个线程访问。
 */
use std::sync::Arc;
// 任务结构体：包含ID和执行闭包（返回结果）
struct Task {
    id: u32,
    func: Box<dyn FnOnce() -> String + Send + 'static>, // 闭包需实现Send
}

// 调度器结构体
struct Scheduler {
    task_tx: Option<channel::Sender<Task>>, // 任务发送端 (使用Option便于take)
    result_rx: Arc<Mutex<channel::Receiver<String>>>, // 结果接收端（共享）
    workers: Option<Vec<thread::JoinHandle<()>>>, // 工作线程句柄队列 (使用Option便于take)
}

impl Scheduler {
    // 创建新调度器
    fn new(num_workers: usize) -> Self {
        let (task_tx, task_rx) = channel::unbounded();
        let (result_tx, result_rx) = channel::unbounded();
        let result_rx = Arc::new(Mutex::new(result_rx));
        let mut workers = vec![];

        // 创建工作线程
        for _ in 0..num_workers {
            // 注意：crossbeam-channel的Receiver支持clone
            let task_rx_clone: channel::Receiver<Task> = task_rx.clone();
            let result_tx: channel::Sender<String> = result_tx.clone();

            let handle = thread::spawn(move || {
                // 循环接收任务并执行
                while let Ok(task) = task_rx_clone.recv() {
                    println!("执行任务{}", task.id);
                    let result = (task.func)(); // 执行任务
                    result_tx.send(result).unwrap(); // 发送结果
                }
            });
            workers.push(handle);
        }

        Scheduler {
            task_tx: Some(task_tx),
            result_rx,
            workers: Some(workers),
        }
    }

    // 添加任务
    fn add_task(&self, id: u32, func: Box<dyn FnOnce() -> String + Send + 'static>) {
        if let Some(ref tx) = self.task_tx {
            tx.send(Task { id, func }).unwrap();
        }
    }

    // 等待所有任务完成并收集结果
    fn wait_and_collect_results(&mut self) -> Vec<String> {
        // 关闭任务发送端，让工作线程接收完毕后退出
        self.task_tx.take(); // 使用take获取所有权并使原位置为None

        // 等待所有工作线程完成
        let mut results = vec![];
        if let Some(workers) = self.workers.take() {
            // 使用take获取所有权
            for handle in workers {
                handle.join().unwrap();
            }
        }

        // 收集所有结果
        let result_rx = self.result_rx.lock().unwrap();
        // 使用iter()并设置超时来获取所有结果，避免无限阻塞
        for result in result_rx.try_iter() {
            results.push(result);
        }
        results
    }
}

fn main() {
    let mut scheduler = Scheduler::new(2); // 创建2个工作线程

    // 添加任务
    for i in 1..=5 {
        scheduler.add_task(
            i,
            Box::new(move || {
                thread::sleep(Duration::from_secs(1)); // 模拟耗时
                format!("任务{}执行完成", i)
            }),
        );
    }

    // 收集结果
    let results = scheduler.wait_and_collect_results();
    println!("所有任务结果：");
    for res in results {
        println!("{}", res);
    }
}
