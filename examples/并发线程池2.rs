use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 测量发送消息的函数执行时间
    let start = Instant::now();
    let (tx, mut rx) = mpsc::channel(32);
    // let rx2 = Arc::new(Mutex::new(rx));

    // let mut handler2 = vec![];

    // for i in 0..10 {
    //     let rx2 = rx2.clone();
    //     handler2.push(tokio::spawn(async move {
    //         loop {
    //             let a = rx2.lock().await.recv().await;
    //             match a {
    //                 None => {
    //                     // let duration = start.elapsed();
    //                     // println!("发送消息耗时: {:?}", duration);
    //                     println!("这个是线程{},退出", i);
    //                     break;
    //                 }
    //                 Some(a) => {
    //                     println!("这个是线程{}", i);
    //                     println!("{:?}", a);
    //                 }
    //             }
    //         }
    //     }));
    // }

    // let mut handler = vec![];
    for i in 0..1000000i64 {
        let tx = tx.clone();
        let a = tokio::spawn(async move {
            tx.send(i).await.unwrap();
        });
        // handler.push(a);
    }

    // for i in handler {
    //     i.await.unwrap();
    // }
    {
        tx
    };
    // for i in handler2 {
    //     i.await.unwrap();
    // }
    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
    let duration = start.elapsed();

    println!("发送消息耗时: {:?}", duration);
}
//
// 发送消息耗时: 57.6611122s
//这个是线程0,退出
// 发送消息耗时: 114.3924263s
