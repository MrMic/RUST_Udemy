// NOTE:
//----------------------------------------------------------------------------
// RESULT:
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
//    }
//  ----------------------------------------------------------------------------

struct Student {
    name: String,
    grade: Option<u32>,
}

// INFO: Check if the student exists in the database
////////////////////////////////////////////////////////////////////////////////////////////////
// fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> { //
//     for student in student_db {                                                            //
//         if student.name == *student_name {                                                 //
//             return Ok(()); // Return unit value                                            //
//         }                                                                                  //
//     }                                                                                      //
//     Err(format!(                                                                           //
//         "Student {} not found in the database.",                                           //
//         student_name                                                                       //
//     )) // Return an error message                                                          //
// }                                                                                          //
//                                                                                            //
// fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {            //
//     for student in student_db {                                                            //
//         if student.name == *student_name {                                                 //
//             return student.grade; // Return the grade if found                             //
//         }                                                                                  //
//     }                                                                                      //
//     None // Return None if the student is not found                                        //
// }                                                                                          //
////////////////////////////////////////////////////////////////////////////////////////////////

fn check_student_get_grade(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade); // Return the grade if found
        }
    }
    Err(format!(
        "Student {} not found in the database.",
        student_name // student name
    )) // Return an error message
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

    // let student_name = String::from("Adam");
    let student_name = String::from("Bob");
    let check_student = check_student_get_grade(&student_name, &student_db);

    match check_student {
        Ok(option_grade) => {
            // let student_grade = get_grade(&student_name, &student_db);
            if let Some(grade) = option_grade {
                println!("{}'s grade is: {}", student_name, grade);
            }
        }
        Err(error_msg) => println!("{error_msg}"),
    }

    /*
    get_grade(&student_name, &student_db)
        .map(|grade| println!("{}'s grade is: {}", student_name, grade))
        .unwrap_or_else(|| println!("{}'s grade is not available.", student_name));

    let grade = get_grade(&student_name, &student_db);
    if let Some(grade) = grade {
        println!("{}'s grade is: {}", student_name, grade);
    } else {
        println!("{}'s grade is not available.", student_name);
    }
    */
}
