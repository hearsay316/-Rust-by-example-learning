// format!("{}",foo) -> "3735928559"
// format("0x{:X}",foo)-> "0xDEADBEEF"
// format("0o{:0}",foo)-> "0o33653337357"
// 根据使用的参数类型是x,o 还是未指定,同样的变量 foo 能够格式化成不能的形式.
// 这个格式化的功能是通过 trait实现的,每种参数类型都对应一种 trait, 最常见的格式化 trait 就是 display,它可以处理参数类型
// 为未指定的情况 ,比如{}'

use std::fmt::{Display, Formatter, Result};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_c = if self.lon >= 0.0 { "E" } else { "W" };
        write!(
            f,
            "{}:{:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:06X}",
            self.red,
            self.green,
            self.blue,
            self.red as u32 * 65536 + self.green as u32 * 256 + self.blue as u32
        )
    }
}
fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 255,
            green: 255,
            blue: 255,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
}
