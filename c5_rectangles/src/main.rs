struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // let width1 = 30;
    // let height2 = 50;

    // let rect1 = (30, 50); //support tuple struct 

    // Refactor to use struct it's more readable
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", 
            area(&rect1)
    );
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}