fn main() {
    let s = String::from("hello");

    take_ownership(s);

    // println!("s After take ownership fn: {}", s); //This would error to ownership, s can not call after the execution.

    let x =5;
    makes_copy(x);

    // println!("x After take ownership fn: {}", x); // x is store on the stack so it's alway callable during the execution.


}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
