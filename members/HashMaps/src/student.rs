use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Student {
    pub name: String,
    pub age: i32,
    pub grade: String,
}

pub fn add_student(
    student_database: &mut HashMap<i32, Student>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    let student = Student { name, age, grade };
    if let Entry::Vacant(e) = student_database.entry(id) {
        e.insert(student);
    } else {
        println!("The id already exist");
    }
}
