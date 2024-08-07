fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I'my a function");
}

fn main() {
    let closure = || println!("I', a closure");
    call_me(closure);
    call_me(function);
}