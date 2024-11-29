use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    // 测量发送消息的函数执行时间
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();

    let mut handler = vec![];
    for num in 0..100 {
        let tx = tx.clone();
        handler.push(thread::spawn(move || {
            for i in 0..100000i64 {
                let tx = tx.clone();
                // println!("线程{}", num);
                tx.send((i, num)).unwrap();
            }
        }));
    }
    {
        tx
    };
    for i in handler {
        i.join().unwrap();
    }

    // for i in handler2 {
    //     i.join().unwrap();
    // }
    while let Ok(message) = rx.recv() {
        println!("GOT = {:?}", message);
    }
    let duration = start.elapsed();

    println!("发送消息耗时: {:?}", duration);
}
// 发送消息耗时: 6.557487s
// 发送消息耗时: 8.341101s
// 发送消息耗时: 48.5475478s
//
// 这个是线程7,退出
// 发送消息耗时: 103.8607842s
