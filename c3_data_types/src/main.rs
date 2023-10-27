use std::io;
fn main() {
    // addtion
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Result in -1

    // remainder
    let remainder = 43 % 5;

    // The Boolean Type
    let t = true;
    let f: bool = false;

    // The Character Type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // get values of tup
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // We can also access a tuple element directly by using a period (.) 
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];

    // Arrays are more useful when you know the number of elements will not need to change.
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize an array to contain the same value for each element 
    let a = [3; 5];

    // Accessing Array Elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // guessing game but get secret from the index
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    
}
