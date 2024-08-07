mod my;
fn function(){
    println!("called `function()`");
}

fn main(){
    my::function();
    my::indirect_access();
    my::nested::function();
}