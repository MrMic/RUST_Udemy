// --------------------------------------
// INFO: Destructured Struct Parameters
// --------------------------------------

struct Point {
    x: i32,
    y: i32,
}

fn print_coord(Point { x, y }: Point) {
    // NOTE: Value p or Point { x: 1, y: 5}
    // NOTE: Pattern: Point {x, y}
    // NOTE: x & y are local variables to the function
    println!("x: {x}, y: {y}");
}

fn print_coord2(Point { x, .. }: Point) {
    println!("x: {x}");
}

fn main() {
    let p = Point { x: 10, y: 20 };
    match p {
        // WARN: Only y is bounded & Destructured, and accessible in the println!()
        Point { x: 0, y } => println!("On the y-axis at y={}", y),
        // WARN: Only x is bounded & Destructured, and accessible in the println!()
        Point { x, y: 0 } => println!("On the x-axis at x={}", x),
        // WARN: As x & y have no value, x & y are bounded & destructured - Not above!
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let p2 = Point { x: 1, y: 5 };
    print_coord(p2);
    let p3 = Point { x: 2, y: 9 };
    print_coord2(p3);
}
