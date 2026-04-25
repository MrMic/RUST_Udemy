/*
* INFO: Ownership
   - Each value in Rust has a variable that's called its owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("s1 is {}", s1);⚠️ Borrow of moved value: `s1`
    println!("s2 is {}", _s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 is {}, s4 is {}", s3, s4);

    {
        // INFO: Inner scope
        let s5 = s3;
        println!("INNER SCOPE => s5 is {}", s5);
    }
    // println!("s5 is {}", s5); WARN: use of moved value: `s5`

    // INFO: Primitive types are Copy - No pointer to the Heap
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);
}
