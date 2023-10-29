/*
This home work convert from Fahrenheit and Celsius
let user input the Fahrenheit and return in Celsius

Formular C = (F - 32) / (9/5)
*/

use std::io;
fn main() {
    let mut f = String::new();
    
    println!("Please input Fahrenheit below 👇");
    io::stdin()
        .read_line(&mut f)
        .expect("Failed to get Fahrenheit");

    let f: f32 = f.trim().parse().expect("Please type a number!");

    let c = (f - 32.0) / 1.8;
    println!("{f} Fahrenheit is: {c} Celsius");
}
