use std::fmt::Debug;
use std::marker::Send;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::{fmt, thread};

pub struct BingFa<T>
where
    T: Add<Output = T> + Copy + Send,
{
    vec: Vec<T>,
    pub sum: Arc<Mutex<T>>,
    channel_number: i32,
    pub history: Vec<T>,
}

impl<T> Debug for BingFa<T>
where
    T: Add<Output = T> + Copy + Send + Debug,
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
    T: Add<Output = T> + Copy + Send + Default + 'static,
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
        let new_sum: T = current_sum.clone();
        self.history.push(new_sum);
        *current_sum = T::default();
        self.vec.clear();
    }
}

//   pub fn set_data_vec_copy(&mut self, vec_chan

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut bing_fa = BingFa::new(vec, 3);
    bing_fa.set_data_vec(vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    println!("{:?}", bing_fa);
    bing_fa.clear();
    bing_fa.set_data_vec(vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    println!("{:?}", bing_fa.sum);
}
