#![allow(unused_variables)]

fn main() {
    // INFO: COMPOUND DATA TYPES

    // Strings
    let fixed_str = "Fixed length string"; // WARN: IMMUTABLE!
    // WARN: String can grow/shrink
    let mut flexible_str = String::from("Flexible length string");
    flexible_str.push_str(" appended");

    // Arrays
    let mut array_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array_1[0]);
    println!("{:?}", array_1);

    // Vectors - WARN: Can grow and shrink!
    let vec_1 = vec![1, 2, 3, 4, 5];

    // Tuples
    let my_info = ("salary", 40000, "Age", 25);
    let (salary, salary_value, age, age_value) = my_info; // WARN: DESTRUCTURING 

    // Empty Tuple - Not cosume any memory
    let unit = ();
}
