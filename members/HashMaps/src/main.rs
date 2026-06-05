////////////////////
// NOTE: HASHMAPS //
////////////////////

mod student;

use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u8> = HashMap::new();
    word_counts.insert("hello", 5);
    word_counts.insert("world", 2);
    word_counts.insert("Rust", 15);
    word_counts.insert("Programming", 5);
    // WARN: Update the value for "Programming"
    word_counts.insert("Programming", 9);

    println!("HashMaps: {:?}", word_counts);

    let has_programming_key = word_counts.contains_key("Programming");
    println!("Contains 'Programming' key: {}", has_programming_key);

    let programming_count = word_counts.get("Programming");
    println!("Value for 'Programming' key: {:?}", programming_count);

    let new_entry = word_counts.entry("C++").or_insert(0);
    println!("Value for 'C++' key after entry: {}", new_entry);
    println!("HashMaps after entry: {:?}", word_counts);

    // WARN: Try to insert "Rust" again with a different value using entry => Not
    // WARN: updated because "Rust" already exists
    word_counts.entry("Rust").or_insert(99);
    println!(
        "HashMaps after trying to insert 'Rust' again: {:?}",
        word_counts
    );

    // NOTE: EXERCISE: Implement a student database using HashMap
    println!("\n--- Student Database ---");

    let mut student_database: HashMap<i32, student::Student> = HashMap::new();
    student::add_student(
        &mut student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );

    student::add_student(
        &mut student_database,
        2,
        String::from("Sarah"),
        16,
        String::from("Grade 10"),
    );

    // Printing the student database

    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}
