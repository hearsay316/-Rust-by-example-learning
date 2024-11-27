use std::fmt::Debug;
use std::marker::Send;
use std::ops::Add;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fmt, thread};
// 求和
#[derive(Default)]
pub struct SeekingHarmony<T>
where
    T: Add<Output = T> + Copy + Send + Default,
{
    vec: Vec<T>,
    pub sum: Arc<Mutex<T>>,
    channel_number: i32,
    pub history: Vec<T>,
}

impl<T> Debug for SeekingHarmony<T>
where
    T: Add<Output = T> + Copy + Send + Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeekingHarmony")
            .field("sum", &self.sum.lock().unwrap().clone())
            .field("channel_number", &self.channel_number)
            .field("history", &self.history)
            .finish()
    }
}
impl<T> SeekingHarmony<T>
where
    T: Add<Output = T> + Copy + Send + Default + 'static + Debug,
{
    pub fn get_vec_channel(&self) -> i32 {
        self.channel_number
    }
    pub fn get_vec(&self) -> Vec<T> {
        self.vec.clone()
    }
    pub fn get_sum(&self) -> T {
        self.sum.lock().unwrap().clone()
    }
    pub fn new(vec: Vec<T>, channel_number: i32) -> Self {
        let mut bing_fa = Self {
            vec,
            sum: Arc::new(Mutex::new(T::default())),
            channel_number,
            history: vec![],
        };
        SeekingHarmony::run_seeking_harmony(&mut bing_fa);
        bing_fa
    }

    fn run_seeking_harmony(&mut self) {
        loop {
            let mut handles = vec![];
            let mut channel_number_clone = self.channel_number;
            if self.vec.len() < self.channel_number as usize {
                channel_number_clone = self.vec.len() as i32;
            }
            let vec_first_10: Vec<T> = self.vec.drain(0..channel_number_clone as usize).collect();
            for i in vec_first_10.clone() {
                let sum = self.sum.clone();
                let handle = thread::spawn(move || {
                    let mut total_sum = sum.lock().unwrap();
                    *total_sum = *total_sum + i;
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            if self.vec.is_empty() {
                break;
            }
        }
    }
    pub fn set_data_vec(&mut self, vec_change: Vec<T>) {
        self.vec.extend(vec_change);
        self.run_seeking_harmony();
    }
    pub fn clear(&mut self) {
        let mut current_sum = self.sum.lock().unwrap();
        self.history.push(current_sum.clone());
        *current_sum = T::default();
    }
}
#[derive(Default)]
pub struct SeekingHarmonyV2<T>
where
    T: Add<Output = T> + Copy + Send + Default,
{
    seeking_harmony: SeekingHarmony<T>,
    tx: Option<mpsc::Sender<Vec<T>>>,
    json_hand: Option<Vec<JoinHandle<()>>>,
}

impl<T> Debug for SeekingHarmonyV2<T>
where
    T: Add<Output = T> + Copy + Send + Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeekingHarmonyV2")
            .field("sum", &self.seeking_harmony)
            .finish()
    }
}
// impl<T> Drop for SeekingHarmonyV2<T>
// where
//     T: Add<Output = T> + Copy + Send + Default,
// {
//     fn drop(&mut self) {
//         let json_hand = std::mem::replace(&mut self.json_hand, None);
//         let tx = std::mem::replace(&mut self.tx, None);
//         // 显式地释放发送者 tx
//         drop(tx.expect("关闭错误"));
//         for worker in json_hand.unwrap() {
//             worker.join().unwrap();
//         }
//     }
// }
fn json_hand_spawn<T>(
    rx: Arc<Mutex<Receiver<Vec<T>>>>,
    sum: Arc<Mutex<T>>,
    channel_number: i32,
) -> Vec<JoinHandle<()>>
where
    T: Add<Output = T> + Copy + Send + Debug + 'static,
{
    let mut join_handle_handles = vec![];

    for _ in 0..channel_number {
        let rx = rx.clone();
        let sum = sum.clone();
        let handler = thread::spawn(move || loop {
            println!("开始");
            let result_i = rx.lock().unwrap().recv();
            match result_i {
                Ok(vec_arr) => {
                    let mut vec: Vec<T> = vec_arr.clone();
                    let mut array_arr = vec![];
                    loop {
                        let mut channel_number_clone = channel_number;
                        if vec.len() < channel_number as usize {
                            channel_number_clone = vec.len() as i32;
                        }
                        let vec_first_10: Vec<T> =
                            vec.drain(0..channel_number_clone as usize).collect();
                        println!("{:?}", vec_first_10);
                        for i in vec_first_10.clone() {
                            let sum = sum.clone();
                            let s = thread::spawn(move || {
                                let mut total_sum = sum.lock().unwrap();
                                *total_sum = *total_sum + i;
                            });
                            array_arr.push(s);
                        }

                        if vec.is_empty() {
                            break;
                        }
                    }
                    for i in array_arr {
                        i.join().unwrap();
                    }
                    println!("结束了");
                }
                Err(_) => {
                    println!("Worker disconnected; shutting down.");
                    break;
                }
            }
        });
        join_handle_handles.push(handler);
    }
    join_handle_handles
}
impl<T> SeekingHarmonyV2<T>
where
    T: Add<Output = T> + Copy + Send + Default + 'static + Debug,
{
    pub fn new(vec: Vec<T>, channel_number: i32) -> Self {
        let (tx, rx) = mpsc::channel::<Vec<T>>();
        let bing_fa = SeekingHarmony::new(vec, channel_number);
        let sum = bing_fa.sum.clone();
        let arc_rx = Arc::new(Mutex::new(rx));
        let json_hand = json_hand_spawn(arc_rx, sum, channel_number);
        Self {
            seeking_harmony: bing_fa,
            tx: Some(tx),
            json_hand: Some(json_hand),
        }
    }
    pub fn set_data_vec(&mut self, vec_change: Vec<T>) {
        self.tx.clone().unwrap().send(vec_change).unwrap();
    }
    pub fn get_seeking_harmony(&mut self) -> &SeekingHarmony<T> {
        let (tx, rx) = mpsc::channel::<Vec<T>>();
        let json_hand = self.json_hand.take();
        let tx2 = std::mem::replace(&mut self.tx, Some(tx));
        drop(tx2.expect("tx2: 错误了"));
        for worker in json_hand.unwrap() {
            // println!("Shutting down worker {}", worker.id);
            worker.join().unwrap();
        }
        let arc_rx = Arc::new(Mutex::new(rx));
        let json_hand = json_hand_spawn(
            arc_rx,
            self.seeking_harmony.sum.clone(),
            self.seeking_harmony.channel_number,
        );
        self.json_hand = Some(json_hand);
        self.info_seeking_harmony()
    }
    fn info_seeking_harmony(&mut self) -> &SeekingHarmony<T> {
        self.seeking_harmony
            .history
            .push(self.seeking_harmony.get_sum());
        &self.seeking_harmony
    }
    pub fn end(self) -> SeekingHarmony<T> {
        let SeekingHarmonyV2 {
            json_hand,
            tx,
            seeking_harmony,
        } = self;
        drop(tx.expect("tx: 错误"));
        // json_hand
        //     .expect("json_hand: 错误")
        //     .join()
        //     .expect("json_hand: 2错误");
        for worker in json_hand.unwrap() {
            // println!("Shutting down worker {}", worker.id);
            worker.join().unwrap();
        }
        seeking_harmony
    }
    pub fn clear(&mut self) {
        self.seeking_harmony.clear();
    }
    pub fn get_vec(&self) -> Vec<T> {
        self.seeking_harmony.get_vec()
    }
    pub fn get_sum(&self) -> T {
        self.seeking_harmony.get_sum()
    }
    pub fn get_vec_channel(&self) -> i32 {
        self.seeking_harmony.get_vec_channel()
    }
    pub fn get_history(&self) -> Vec<T> {
        self.seeking_harmony.history.clone()
    }
}
//   pub fn set_data_vec_copy(&mut self, vec_chan

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut bing_fa = SeekingHarmonyV2::new(vec, 3);
    // println!("{:?}", bing_fa);
    // bing_fa.set_data_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // bing_fa.set_data_vec(vec![9, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // bing_fa.set_data_vec(vec![8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // bing_fa.set_data_vec(vec![7, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // thread::sleep(Duration::from_secs(3));
    // println!("{:?}", bing_fa);
    // let s = bing_fa.get_seeking_harmony();
    // println!("{:?}", s);
    // bing_fa.set_data_vec(vec![7, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // let s = bing_fa.get_seeking_harmony();
    // println!("{:?}", s);
    let end = bing_fa.end();
    println!("{:?}", end);
}
