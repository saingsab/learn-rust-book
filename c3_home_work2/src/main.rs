/*
Find and Print Nth Fibonacci Numbers
In mathematical terms, the sequence Fn of Fibonacci numbers is defined by the recurrence relation: 
F_{n} = F_{n-1} + F_{n-2} 
with seed values and F_0 = 0 and F_1 = 1 

The Nth Fibonacci Number can be found using the recurrence relation shown above:

if n = 0, then return 0. 
If n = 1, then it should return 1. 
For n > 1, it should return Fn-1 + Fn-2
*/
use std::io;
fn main() {
    let mut n = String::new();

    println!("Please input the N of Fibonacci below: 👇");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to get N of Fibonacci");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    let mut f= 0;
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    if n == 0 {
       f = 0;
    } else if n == 1 || n == 2 {
        f = 1;
    } else {
        // 0 1 1 2 3 5 8 13 
        for number in (2..n) {
            println!("{number}");
            // b + (number - 1)

            f += b + (number - 1);
        }
    }
    println!("{n}th of Fibonacci is: {f}");
}
