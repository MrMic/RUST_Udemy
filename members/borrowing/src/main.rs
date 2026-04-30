// INFO:
// -------------------------------------------------------------------
//          ╭─────────────────────────────────────────────────────────╮
//          │                        BORROWING                        │
//          ╰─────────────────────────────────────────────────────────╯
// - Borrowing Rules:
// 1. At any given time, you can have either one mutable reference or any number
//  of immutable references.
//  2. References must always be valid.
//
// INFO:
// --------------------------------------------------------------------
//  - Solve out 2 problems:
//  1. Data races
//  2. Dangling references
// --------------------------------------------------------------------

fn main() {
    let mut vec_1 = vec![1, 2, 3];

    // WARN:
    // - Non-Lexical Lifetimes (NLL): borrows end at last use, not end of scope.
    // Without the println!, ref1 is never used after creation → its borrow ends
    // here, before ref2 is created → no overlap → compiles fine.
    // - With the println!, both ref1 and ref2 must be alive at the same time
    // (same statement) → two active &mut → borrow checker error.
    let ref1 = &mut vec_1; // borrow starts — ends here if ref1 is never used again
    // let ref2 = &mut vec_1; // OK only if ref1's borrow already ended
    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2); // both alive here → ERROR
    println!("ref1: {:?}", ref1);

    let ref3 = &vec_1; // OK: immutable borrow starts here
    let ref4 = &vec_1;
    println!("ref3: {:?}, ref4: {:?}", ref3, ref4);

    // WARN: Scope of ref3 & ref4 end here, so we can create a new mutable reference after this point.
    let ref5 = &mut vec_1; // OK: mutable borrow starts here, after ref3 & ref4's borrows ended

    let vec_3 = {
        let vec_2 = vec![4, 5, 6];
        // &vec_2 // FIXME: vec_2 is dropped at the end of this block, so ref to it becomes dangling
    };
}
