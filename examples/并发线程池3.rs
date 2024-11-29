use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    // 测量发送消息的函数执行时间
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();
    let rx2 = Arc::new(Mutex::new(rx));

    let mut handler2 = vec![];

    for i in 0..10 {
        let rx2 = rx2.clone();
        handler2.push(thread::spawn(move || {
            loop {
                let a = rx2.lock().unwrap().recv();
                match a {
                    Err(_) => {
                        // let duration = start.elapsed();
                        // println!("发送消息耗时: {:?}", duration);
                        println!("这个是线程{},退出", i);
                        break;
                    }
                    Ok(a) => {
                        println!("这个是线程{}", i);
                        println!("{:?}", a);
                    }
                }
            }
        }));
    }

    // let mut handler = vec![];
    for i in 0..1000000i64 {
        let tx = tx.clone();
        // thread::spawn(move || {
        //
        // });
        tx.send(i).unwrap();
    }
    {
        tx
    };
    // for i in handler {
    //     i.await.unwrap();
    // }

    for i in handler2 {
        i.join().unwrap();
    }
    // while let Ok(message) = rx.recv() {
    //     println!("GOT = {}", message);
    // }
    let duration = start.elapsed();

    println!("发送消息耗时: {:?}", duration);
}
// 发送消息耗时: 6.557487s
// 发送消息耗时: 8.341101s
// 发送消息耗时: 48.5475478s
//
// 这个是线程7,退出
// 发送消息耗时: 103.8607842s
