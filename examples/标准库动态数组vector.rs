fn   main(){

    let collected_iterator :Vec<_> = (0..10).collect();
    println!("Collected (0..10) into :{:?}",collected_iterator);
    // 可以用 vec! 宏来初始化一个vector
    let mut xs = vec![1i32,2,3];
    println!("initial vector :{:?}",xs);
    println!("push 3 into the vector");
    xs.push(4);
    println!("Vector :{:?}",xs);

    // 错误
    // collected_iterator.push(0);
    println!("Vector size :{}",xs.len());
    println!("Second element:{}",xs[1]);
    println!("Pop last element :{:?}",xs.pop().unwrap());
    // println!("Fourth element:{}",xs[3]);

    println!("Contents of xs:");
    for x in xs.iter(){
        println!("> {}",x);
    }
    for(i,x) in xs.iter().enumerate(){
        println!("In position {} we have value {}",i,x);
    }
    for x in xs.iter_mut(){
        *x*=3;
    }
    println!("Updated vector :{:?}",xs);
}