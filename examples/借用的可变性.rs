#[derive(Clone,Copy)]
struct Book {
    // &' static str 是一个对分配在只读内存区的字符串
    aurhor:&'static str,
    title:&'static str,
    year:u32
}

fn borrow_book(book:&Book){
    println!("I immutably borrowed {} -{} edition",book.title,book.year);
}

fn new_edition(book:&mut Book){
    book.year = 2004;
    println!("I mutably borrowed {} -{} edition",book.title,book.year);
}
fn main(){
    let immutabook = Book{
        aurhor:"Douglas Hofstadter",
        title:"Escher,Bach",
        year:1979
    };
  let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);

}