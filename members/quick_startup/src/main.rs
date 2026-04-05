#![allow(unused_variables)]

/*
    - Functions
    - Code Blocks
*/
// ______________________________________________________________________
fn my_fn(s: &str) {
    println!("{s}!");
}

// ______________________________________________________________________
fn multiplication(x: i32, y: i32) -> i32 {
    println!("Multiplying {x} * {y}"); // INFO: STATEMENT
    x * y // INFO: EXPRESSION
}

// ______________________________________________________________________
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32, f64) {
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 as f64 / num2 as f64;
    (sum, difference, product, quotient)
}

// INFO: EXPRESSION vs STATEMENTS _______________________________________
// INFO: EXPRESSION is something that evaluates to a valid value.
// INFO: STATEMENT is something that performs an action but does not return a value => a unit value

// ______________________________________________________________________
fn main() {
    // my_fn("This my function");
    let str = "This is my function";
    my_fn(str);

    let result = multiplication(5, 10);
    println!("The result is: {result}");

    let (sum, difference, product, quotient) = basic_math(10, 5);
    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");

    // INFO: Code Blocks
    let full_name = {
        let first_name = "John";
        let last_name = "Doe";
        println!("First Name: {first_name}");
        // INFO: Code block can also return values like functions
        format!("{first_name} {last_name}")
    };
    // println!("First Name: {first_name}"); // Inaccessible outside the block
}
