fn main(){
    let mut count= 0u32;
    println!("让我们数到无穷大  Let's count until infinity!");
    loop{
        println!("{}",count);
        count +=1;
        if count ==3{
            println!("跳过这次迭代的剩下内容");
           continue;
        }
        println!("{}",count);
        if count==5{
            println!("这个是要退出循环了");
            break;
        }
    }
}