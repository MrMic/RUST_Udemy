/*
* INFO: ------------------------------------
* Ownership in Functions
*  - Functions that take ownership
*  - Functions that give ownership
*  - Functions that take and gives ownership
*  -----------------------------------------
*/

fn main() {
    // INFO:Functions that take ownership
    let vec_1 = vec![1, 2, 3];
    // takes_ownership(vec_1);  WARN: similar as vec = vec_1
    // println!("The vector: {:?} has not been moved", vec_1); WARN: Borrow of moved value
    takes_ownership(vec_1.clone());

    // INFO: Functions that give ownership back
    println!("-----------------------------\n");
    let vec_2 = gives_ownership();
    println!("--> 👉️ vec_2 is now: {:?}", vec_2);

    // INFO: Functions that take and returns ownership
    println!("-----------------------------\n");
    let vec_3 = takes_and_returns_ownership(vec_2);
    // println!("--> 👉️ vec_2 is now: {:?}", vec_2); WARN: Borrow of moved value
    println!("--> 👉️ vec_3 is now: {:?}", vec_3);

    // INFO: Functions that take **Stack** Data
    println!("-----------------------------\n");
    let var = 42;
    println!("--> 👉️ var was: {}", var);
    stack_function(var);
    println!("--> 👉️ var in main, is now: {}", var);
}

// ______________________________________________________________________
fn takes_ownership(vec: Vec<i32>) {
    println!("--> 👉️ The vector: {:?} has been moved", vec);
}

// ______________________________________________________________________
fn gives_ownership() -> Vec<i32> {
    println!("gives_ownership!");
    vec![4, 5, 6]
}

// ______________________________________________________________________
fn takes_and_returns_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(7);
    vec
}

// ______________________________________________________________________
fn stack_function(mut var: i32) {
    var = 56;
    println!("--> 👉️ var in stack_function, is now: {}", var);
}
