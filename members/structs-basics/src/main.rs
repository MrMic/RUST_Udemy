/* INFO:
 * ----------------------------------------------------------------------------
//          ╭─────────────────────────────────────────────────────╮
//          │                  STRUCTS BASICS                     │
//          ╰─────────────────────────────────────────────────────╯
 *  - Named-field structs
 *  - Tuple structs
 *  - Unit-like structs
 * -----------------------------------------------------------------------------
 */

// NOTE: Struct = group of related data of different types

// NOTE: Named-field struct
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

struct Point2D(i32, i32); // Tuple struct
struct Point3D(i32, i32, i32); // Tuple struct
fn main() {
    let mut my_car = Car {
        owner: String::from("Alice"),
        year: 2020,
        fuel_level: 75.5,
        price: 25_000,
    };
    println!(
        "My car details => Owner: {}, Year: {}, Fuel Level: {}%, Price: ${}",
        my_car.owner, my_car.year, my_car.fuel_level, my_car.price
    );
    my_car.fuel_level -= 10.0; // Decrease fuel level by 10%

    // WARN: Partial move => Some portion of data has moved out of the struct instance
    // let extracted_owner = my_car.owner; // WARN: Move ownership of the owner field to extracted_owner
    // println!("Extracted Owner: {}", my_car.owner); // ERROR: my_car.owner is no longer valid after the move
    let extracted_owner = my_car.owner.clone(); // Clone the owner field to keep my_car valid

    let another_car = Car {
        // owner: String::from("Bob"),
        ..my_car // WARN: Struct update syntax => Creates a new instance by copying remaining fields from my_car
    };
    // println!("Owner is: {}", my_car.owner); // ERROR: copy of my_car is moved to another_car, so my_car is no longer valid

    // ╾────────────────────────────────────────────────────────────────────╼
    // NOTE: Tuple struct
    let point_2D = (1, 2); // Tuple
    let point3D = (3, 4, 5); // Tuple

    let point1 = Point2D(1, 2);
    let point2 = Point3D(3, 4, 5);
    println!("Point 2D: ({}, {})", point1.0, point1.1);

    // ╾────────────────────────────────────────────────────────────────────╼
    // NOTE: Unit-like struct (struct without fields)
    struct ABC;
}

// ______________________________________________________________________
fn print_tuples_coordinates(coords: Point2D) {
    println!("Coordinates: ({}, {})", coords.0, coords.1);
}
