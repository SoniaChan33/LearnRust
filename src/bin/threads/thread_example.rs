use core::num;
use core::str;
use std::result;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::thread;
use std::time::Duration;
use std::vec;
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("spawn thread: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("main thread: {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();
}

#[test]
fn channel_study() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 发送值

    for receive in rx {
        println!("Got: {}", receive);
    }
}

#[test]
fn mutex_example() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn move_example() {
    let s = String::from("hello");
    // 错误：子线程可能比主线程生命周期长，s可能被提前释放
    thread::spawn(move || {
        println!("{}", s); // 编译错误
    });
}

#[test]
fn more_sender_example() {
    let (tx, rx) = mpsc::channel();
    let tx2: mpsc::Sender<String> = tx.clone();

    // 线程1:发送数据
    thread::spawn(move || {
        let val: String = String::from("hi");
        tx.send(val).unwrap();
    });

    // 线程2：发送数据
    thread::spawn(move || {
        let val: String = String::from("hello");
        tx2.send(val).unwrap();
    });

    // 接收数据
    for received in rx {
        println!("Got: {}", received);
    }
}
