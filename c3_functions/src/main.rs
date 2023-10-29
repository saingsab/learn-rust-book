fn main() {
    println!("Hello, world!");

    // calling another_function
    another_function();

    // assign parameter
    function_with_param(5);

    // multiple param
    multiple_param(5, 10);

    // call fn expression
    function_expression();

    // calling Functions with Return Values
    let x = five();
    println!("The value of x is: {x}");
}

// Declear function
fn another_function() {
    println!("This is another function");
}

// Parameters
fn function_with_param(x: i32) {
    println!("Function with Parameter x: {x}")
}

// multiple parameters, 
// separate the parameter declarations with commas
fn multiple_param(x: i32, y: i32) {
    println!("multiple param the valu of x: {x}, and the value of y: {y}");
}

// Expressions do not include ending semicolons
fn function_expression() {
    let y = {
        let x = 3;
        x + 1 //no semicolns it's is an expression
    };

    println!("The value of y is: {y}");
}

// Functions with Return Values
fn five() -> i32 {
    5
}