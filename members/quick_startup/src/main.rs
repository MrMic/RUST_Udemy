#![allow(unused_variables)]

/*
    Loops

*/

fn main() {
    // WARN: start..end or start..=end is only computed when start < end.
    // WARN: If start >= end, the *range* will be empty and the loop will not execute.
    // INFO: Iterating over a range of values in a ascending order
    for i in 0..=5 {
        println!("i: {}", i);
    }

    println!("-------------------------------");
    // INFO: Iterating over a *range* of values in a descending order
    for i in (0..=5).rev() {
        println!("i: {}", i);
    }

    println!("-------------------------------");
    // INFO: Iterating over a *range* of values with a step
    for i in (0..=10).step_by(2) {
        println!("i: {}", i);
    }

    println!("-------------------------------");
    // INFO: Iterating over a *range* of values with a step in reverse order
    for i in (0..=10).rev().step_by(2) {
        println!("i: {}", i);
    }

    // WARN: for i in 0.0..=5.0 { ... } is not valid because ranges in Rust only work with integer types.

    // INFO: .. is a valid type.
    let range = ..;

    println!("-------------------------------");
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    for (number, word) in pairs {
        println!("{}: {}", number, word);
    }
}
