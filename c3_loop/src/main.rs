fn main() {
    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Lable loop for multiple loop so that we can identify
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2  {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut number = 3;
    while number != 0{

        println!("number is: {number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }

    // rang default std .rev()  for countdown
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

}
