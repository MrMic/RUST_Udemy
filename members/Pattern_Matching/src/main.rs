// ----------------------------------------
// INFO: PATTERN MATCHING CONTEXTS
// ----------------------------------------

#[allow(irrefutable_let_patterns, unused_variables)]
fn main() {
    // INFO: 1. Match Expression
    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something Else"),
    }
    // NOTE: Value: x
    // NOTE: Pattern: 1,2,3,_

    // INFO: 2. If let expression
    let x = 3;
    if let 5 = x {
        // NOTE: if x == 5
        println!("Matched five");
    }
    // NOTE: Value: x
    // NOTE: Pattern: 5

    if let x = 5 {
        // NOTE: let x = 5
        println!("This always run");
        println!("x inner: {x}");
    }
    println!("x outer: {x}");

    // INFO: Binding Pattern
    // NOTE: Value: Concrete Value
    // NOTE: Pattern: Variable

    // INFO: 3. while let
    let numbers = [1, 2, 2, 3];
    let mut i = 0;
    while let 2 = numbers[i] {
        println!("Found a value 2 at index : {}", i);
        i += 1;
    }
    // NOTE: Value: numbers[1]
    // NOTE: Pattern: 2

    // INFO: 4. let binding
    let (a, b) = (10, 20);
    // NOTE: Value: (10, 20)
    // NOTE: Pattern: (a, b)

    // INFO: 5. Function Parameters
    let point = (10, 20);
    print_coords(point);
    // NOTE: Value: (10, 20)
    // NOTE: Pattern: (x, y)
    // NOTE: Type: (i32, i32)
}

fn print_coords((x, y): (i32, i32)) {
    println!("x: {x}, y: {y}");
}
