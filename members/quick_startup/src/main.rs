#![allow(unused_variables)]

/*
    Conditional
    - If else
    - If else if ladder
    - Match

*/

fn main() {
    // Match
    let marks = 95;
    // let mut grade = 'N';

    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'N', // Default case
    };

    println!("Grade: {}", grade);
}
