// super  self
fn function() {
    println!("called `function()`");
}
mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}
mod my {
    fn function() {
        println!("called `my::function()`");
    }
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
            super::super::function();
        }
    }
    pub fn indirect_call() {
        print!("called `my::indirect_call()`,that\n>");
        // `self`关键字表示当前的模块用作域--在这个例子是`my`
        // 调用 `self::function()`和直接调用`function()` 都的到相同的结果
        // 因为他们是相同的函数
        self::function();
        function();
        // 我们也可以使用`self`来访问 `my`内部的另外一个模块
        self::cool::function();
        // super 关键字表示父作用域(在`my`模块外面)
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
