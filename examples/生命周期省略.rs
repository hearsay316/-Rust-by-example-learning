fn elided_input(x:&i32){
    println!("`elided_input`:{}",x);
}
fn annotated_input<'a>(x:&'a i32){
    println!("`annotated_input`:{}",x);
}
fn elded_pass(x:&i32)->&i32{x}

fn annoted_pass<'a>(x:&'a i32)->&'a i32{x}

fn main(){
    let x = 3;
    elided_input(&x);
    annoted_pass(&x);
    println!("`elided_pass`:{}",elded_pass(&x));
    println!("`annotated_pass`:{}",annoted_pass(&x));

}