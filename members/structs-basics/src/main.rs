#![allow(dead_code)]

/* INFO:
 * ----------------------------------------------------------------------------
 *  - Adding functionality to Structs.
 * -----------------------------------------------------------------------------
 */

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

// WARN: Two requirements for a function to be considered as method:
//     1. It must be defined within an impl block.
//     2. It must have &self as its first parameter, which represents
//        the instance of the struct on which the method is called.
impl Car {
    // ______________________________________________________________________
    // INFO: immutable reference to self
    fn display_car_info(&self) {
        println!("Owner: {}", self.owner);
        println!("Year: {}", self.year);
        println!("Fuel Level: {}", self.fuel_level);
        println!("Price: ${}", self.price);
    }

    // ______________________________________________________________________
    // INFO: mutable reference to self
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
        println!(
            "  Refueled {} units => New fuel level: {}",
            gallons, self.fuel_level
        );
    }

    // ______________________________________________________________________
    // INFO: Owned form of self (consumes the instance)
    fn sell(self, new_owner: String) -> Self {
        println!("  => 💰 Car sold from {} to {}", self.owner, new_owner);
        Car {
            owner: new_owner,
            year: self.year,
            fuel_level: self.fuel_level,
            price: self.price,
        }
    }

    // INFO: Associated function (not a method, does not take self)
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    // INFO: Associated function: new 'constructor'
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year,
            fuel_level: 0.0,
            price: 0,
        }
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2020,
        fuel_level: 0.0,
        price: 5_000,
    };

    // Display initial car info
    my_car.display_car_info();

    println!("-----------------------------");
    // Refuel the car
    my_car.refuel(10.0);

    println!("-----------------------------");
    // Sold the car to a new owner
    let new_car = my_car.sell(String::from("XYZ"));
    // Display updated car info after selling
    println!();
    new_car.display_car_info();

    println!("-----------------------------");
    let new_car = Car::new(String::from("DEF"), 2021);
    new_car.display_car_info();
}
