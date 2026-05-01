// INFO:
// -------------------------------------------------------------------
//          ╭─────────────────────────────────────────────────────────╮
//          │                        BORROWING                        │
//          ╰─────────────────────────────────────────────────────────╯
//  Borrowing in Functions:
//    - Functions that immutably borrow values
//    - Functions that mutably borrow values
//    - Functions that not uses borrow but return it
//    - Functions that uses mixed types of borrows
//
// --------------------------------------------------------------------

// ______________________________________________________________________
fn borrows_vec(vec: &Vec<i32>) {
    println!("Borrowed vector: {:?}", vec);
}

// ______________________________________________________________________
fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
    println!("Mutably borrowed vector: {:?}", vec);
}

// ______________________________________________________________________
fn mixed_borrows(subject: &String, scores: &mut Vec<i32>) {
    println!("{} before update: {:?}", subject, scores);
    scores.push(100);
    println!("{} after update: {:?}", subject, scores);
}

// ──────────────────────────────────────────────────────────────────────
fn main() {
    // NOTE: Functions that immutably borrow values
    let vec1 = vec![1, 2, 3];
    let ref1 = &vec1;
    borrows_vec(ref1);
    println!("Original vector after borrowing: {:?}", vec1);

    println!("---------------------------------------------");
    // NOTE: Functions that mutably borrow values
    let mut vec2 = vec![4, 5, 6];
    let ref2 = &mut vec2;
    mutably_borrows_vec(ref2);
    println!("Original vector after mutable borrowing: {:?}", vec2);

    // NOTE: Functions that not uses borrow but return it
    // Will be covered later under the topic of lifetimes

    println!("---------------------------------------------");
    // NOTE: Functions that uses mixed types of borrows
    let subject = String::from("Math");
    let mut scores = vec![85, 90, 95];
    mixed_borrows(&subject, &mut scores);
}
