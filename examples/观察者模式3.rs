use std::fmt::Debug;
use std::marker::Send;
use std::ops::Add;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fmt, thread};

#[derive(Default)]
pub struct BingFa<T>
where
    T: Add<Output = T> + Copy + Send + Default,
{
    vec: Vec<T>,
    pub sum: Arc<Mutex<T>>,
    channel_number: i32,
    pub history: Vec<T>,
}

impl<T> Debug for BingFa<T>
where
    T: Add<Output = T> + Copy + Send + Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BingFa")
            .field("sum", &self.sum.lock().unwrap().clone())
            .field("channel_number", &self.channel_number)
            .field("history", &self.history)
            .finish()
    }
}
impl<T> BingFa<T>
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
        BingFa::run_bing_fa(&mut bing_fa);
        bing_fa
    }

    fn run_bing_fa(&mut self) {
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
        self.run_bing_fa();
    }
    pub fn clear(&mut self) {
        let mut current_sum = self.sum.lock().unwrap();
        self.history.push(current_sum.clone());
        *current_sum = T::default();
    }
}
#[derive(Default)]
pub struct BingFaV2<T>
where
    T: Add<Output = T> + Copy + Send + Default,
{
    bing_fa: BingFa<T>,
    tx: Option<mpsc::Sender<Vec<T>>>,
    json_hand: Option<thread::JoinHandle<()>>,
}

impl<T> Debug for BingFaV2<T>
where
    T: Add<Output = T> + Copy + Send + Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BingFaV2")
            .field("sum", &self.bing_fa)
            .finish()
    }
}
// impl<T> Drop for BingFaV2<T>
// where
//     T: Add<Output = T> + Copy + Send + Default,
// {
//     fn drop(&mut self) {
//         let json_hand = std::mem::replace(&mut self.json_hand, None);
//
//         let tx = std::mem::replace(&mut self.tx, None);
//         // 显式地释放发送者 tx
//         drop(tx.expect("xsxsx"));
//
//         // 等待 json_hand 线程结束，并处理可能的错误
//         if let Err(e) = json_hand.expect("xsxsxs").join() {
//             eprintln!("Error joining thread: {:?}", e);
//         }
//     }
// }
fn json_hand_spawn<T>(
    rx: Receiver<Vec<T>>,
    sum: Arc<Mutex<T>>,
    channel_number: i32,
) -> JoinHandle<()>
where
    T: Add<Output = T> + Copy + Send + Debug + 'static,
{
    thread::spawn(move || {
        println!("开始");
        for i in rx.iter() {
            let mut vec: Vec<T> = i.clone();
            let mut array_arr = vec![];
            loop {
                let mut channel_number_clone = channel_number;
                if vec.len() < channel_number as usize {
                    channel_number_clone = vec.len() as i32;
                }
                let vec_first_10: Vec<T> = vec.drain(0..channel_number_clone as usize).collect();
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
    })
}
impl<T> BingFaV2<T>
where
    T: Add<Output = T> + Copy + Send + Default + 'static + Debug,
{
    pub fn new(vec: Vec<T>, channel_number: i32) -> Self {
        let (tx, rx) = mpsc::channel::<Vec<T>>();
        let mut bing_fa = BingFa::new(vec, channel_number);
        let sum = bing_fa.sum.clone();
        let json_hand = json_hand_spawn(rx, sum, channel_number);
        let mut data = Self {
            bing_fa,
            tx: Some(tx),
            json_hand: Some(json_hand),
        };
        data
    }
    fn set_data_vec(&mut self, vec_change: Vec<T>) {
        self.tx.clone().unwrap().send(vec_change).unwrap();
    }
    fn get_bing_fa(&mut self) -> &BingFa<T> {
        let (tx, rx) = mpsc::channel::<Vec<T>>();
        let json_hand = std::mem::replace(&mut self.json_hand, None);

        let tx2 = std::mem::replace(&mut self.tx, Some(tx));
        drop(tx2.expect("tx错误了"));
        json_hand.unwrap().join().expect("swswsd");
        let json_hand = json_hand_spawn(rx, self.bing_fa.sum.clone(), self.bing_fa.channel_number);
        let _ = std::mem::replace(&mut self.json_hand, Some(json_hand));
        self.info_bing_fa()
    }
    pub fn info_bing_fa(&mut self) -> &BingFa<T> {
        let sum = self.bing_fa.get_sum();
        let mut history = self.bing_fa.history.clone();
        history.push(sum.clone());
        self.bing_fa.history = history.clone();
        &self.bing_fa
    }
    fn end(self) -> BingFa<T> {
        let BingFaV2 {
            json_hand,
            tx,
            bing_fa,
        } = self;
        drop(tx.expect("tx: 错误"));
        json_hand
            .expect("json_hand: 错误")
            .join()
            .expect("json_hand: 2错误");

        bing_fa
    }
    pub fn clear(&mut self) {
        self.bing_fa.clear();
    }
    pub fn get_vec(&self) -> Vec<T> {
        self.bing_fa.get_vec()
    }
    pub fn get_sum(&self) -> T {
        self.bing_fa.get_sum()
    }
    pub fn get_vec_channel(&self) -> i32 {
        self.bing_fa.get_vec_channel()
    }
    pub fn get_history(&self) -> Vec<T> {
        self.bing_fa.history.clone()
    }
}
//   pub fn set_data_vec_copy(&mut self, vec_chan

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut bing_fa = BingFaV2::new(vec, 3);
    println!("{:?}", bing_fa);
    bing_fa.set_data_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    bing_fa.set_data_vec(vec![9, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    bing_fa.set_data_vec(vec![8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    bing_fa.set_data_vec(vec![7, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // thread::sleep(Duration::from_secs(3));
    println!("{:?}", bing_fa);
    let s = bing_fa.get_bing_fa();

    println!("{:?}", s);
    bing_fa.set_data_vec(vec![7, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let s = bing_fa.get_bing_fa();

    let end = bing_fa.end();
    println!("{:?}", end);
}
