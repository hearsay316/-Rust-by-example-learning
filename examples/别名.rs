type NanSecond = u64;
type Inch = u64;
#[allow(non_camel_case_types)]
type u64_t = u64;
fn main() {
    let nanosecond: NanSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!(
        "{} nanosecond + {} inches = {} unit?",
        nanosecond,
        inches,
        nanosecond + inches
    );
}

/*
类型转换
Rust 使用 trait 解决类型之间的转换问题。
最一般的转换会用到 From 和 Into 两个 trait。
不过，即便常见的情况也可能会用到特别的 trait，
尤其是从 String 转换到别的类型，以及把别的类型转换到 String 时。
*/

/*
1 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51692 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51697 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51698 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51704 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51706 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51713 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51715 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51722 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51724 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51731 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51733 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51742 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51740 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51749 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51751 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51758 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51760 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51769 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51770 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51778 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51779 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51787 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51788 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51796 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51797 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51805 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51806 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51814 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51815 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51821 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51824 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51830 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51833 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51839 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51842 accepted udp:110.242.72.55:11711 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51850 accepted udp:47.97.126.85:3000 [socks -> direct]
2024/07/30 00:02:26 udp:192.168.1.8:51851 accepted udp:110.242.72.55:11711 [socks -> direct]



*/
