// NOTE:
// -----------------------------------------------------------------------------
// OPTION
// -----------------------------------------------------------------------------

struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade; // Return the grade if found
        }
    }
    None // Return None if the student is not found
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(85),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(92),
        },
        Student {
            name: String::from("Charlie"),
            grade: None, // Charlie's grade is not available
        },
    ];

    let student_name = String::from("Bob");

    /*
    get_grade(&student_name, &student_db)
        .map(|grade| println!("{}'s grade is: {}", student_name, grade))
        .unwrap_or_else(|| println!("{}'s grade is not available.", student_name));
    */
    let grade = get_grade(&student_name, &student_db);
    if let Some(grade) = grade {
        println!("{}'s grade is: {}", student_name, grade);
    } else {
        println!("{}'s grade is not available.", student_name);
    }
}
