#![allow(unused_variables)]

// NOTE:
//-------------------------------------------
// Enums
// --------------------------------------------
// Enums: Multiples variants of a type

// WARN: Enums vs Structs
// Structs fields have types
#[allow(dead_code)]
enum WeekDays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[allow(dead_code)]
enum TravelTypes {
    Aeroplane(f32), // miles
    Car(f32),       // miles
    Train(f32),     // miles
}

impl TravelTypes {
    fn travel_allowance(&self) -> f32 {
        match self {
            TravelTypes::Car(miles) => miles * 2.0,
            TravelTypes::Train(miles) => miles * 3.0,
            TravelTypes::Aeroplane(miles) => miles * 5.0,
        }
    }
}

fn main() {
    let participant: TravelTypes = TravelTypes::Car(60.0);
    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance()
    );
}
