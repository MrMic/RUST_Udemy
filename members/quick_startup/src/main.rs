#![allow(unused_variables)]

/*
    Loops

*/

fn main() {
    'outer: loop {
        loop {
            println!("This is an infinite loop");
            break 'outer; // Exit the outer loop
        }
    }

    let a = loop {
        println!("This loop will return a value");
        break 42; // Return the value 42 from the loop
    };

    // while loop
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // INFO: Compound Data: Arrays and Tuples - Fixed-size collections of values
    // INFO: Collections: Vectors and HashMaps - Dynamic collections of values (grow/shrink)
}
