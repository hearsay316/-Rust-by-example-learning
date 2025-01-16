use std::env;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::System::Console::FreeConsole;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--background".to_string()) {
        unsafe { FreeConsole() };
        // 后台运行的逻辑
    } else {
        // 正常运行的逻辑
    }
    sleep(Duration::from_secs(5 * 60))
}
