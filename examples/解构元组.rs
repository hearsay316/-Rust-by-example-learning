// 元组 可以在match 中解构,如下所示
fn main() {
    /*
    letletletleltletletletleltletletleltletletlletletleltletletleltletllteletllletleltletletletlletletlleltelteltletletleltletlletlletletlletletleltletletletletlelttlelt

    let let let; let let let;let let let let let let let let letletletlet

    */
    let triple = (0, -2, 3);
    println!("Tell me about :{:?}", triple);
    match triple {
        (0, x, y) => {
            println!("第一个元素是:{},  第二个元素是{} , ", x, y);
        }
        (1, ..) => {
            println!(" 这个是计算啥");
        }
        _ => {
            println!(" 这个是最后的else")
        }
    }
}
