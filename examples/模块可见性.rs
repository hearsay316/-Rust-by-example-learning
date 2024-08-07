mod my_mod {
    fn private_function(){
        println!("called `my_mod::private_function()`");
    }
    pub fn function(){
        private_function();
        println!("called `my_mod::function()`");
    }
    pub fn indireact_access(){
        print!("called `my_mod::indireact_access()`,that\n>");
    }
    pub mod nested{
        pub fn funtion(){
            println!("called `my_mod::nested::function()`");
            private_function();
        }
        #[allow(dead_code)]
        fn private_function(){
            println!("called `my_mod::nested::private_function()`");
            public_function_in_super_mod();
            public_function_in_my_mod();
        }
        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            print!("called `my_mod::nested::public_function_in_my_mod()`,that\n > ");
            public_function_in_nested()
        }
        pub(self) fn public_function_in_nested(){
            println!("called `mu_mod::nested::public_function_in_nested`");
        }
        pub(super) fn public_function_in_super_mod(){
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub(super) fn call_public_function_in_my_mod(){
        print!("called `my_mod::call_public_function_in_my_mod()`,that\n>");
        nested::public_function_in_my_mod();
        print!(">");
        nested::public_function_in_super_mod()
    }
    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
        public_function_in_crate2();
    }
    pub(crate) fn public_function_in_crate2(){
        println!("called `my_mod::public_function_in_crate2()`");
    }
    mod private_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("called `my_mod::private_nested::function()`")
        }
    }
}
fn function(){
    println!("called `function()`");
}
fn  main(){
    function();
    my_mod::function();
    my_mod::public_function_in_crate();
    // 公有项,包括嵌套模块内的,都可以在夫模块外包访问.
    // my_mod::indireact_access();
    // my_mod::nested::funtion();
    // my_mod::call_public_function_in_my_mod();
    // // pub(crate)   项 可以在同一个crate 中的任何地方访问
    // my_mod::public_function_in_crate();
    // my_mod::nested::public_function_in_my_mod();
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // Error! `private_nested` is a private module
    // my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}