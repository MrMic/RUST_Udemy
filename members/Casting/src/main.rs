// INFO: Casting between references
// INFO: Assignment of references

fn main() {
    // INFO: Casting between references
    let x = 5;
    let y = x as f32; // Cast i32 to f32
    println!("x = {x}, y = {y}");

    // WARN: Casting immutable reference -> mutable reference is not allowed
    let data = 42;
    let immutable_ref = &data; // Immutable reference

    // WARN: Casting mutable reference -> immutable reference is allowed
    let mut data = 42;
    let mutable_ref = &mut data; // Mutable reference
    let immutable_ref_from_mutable = mutable_ref as &i32; // Cast mutable reference

    // INFO: Reborrowing
    // WARN:  *mutable_ref = 43;
    // INFO: not allowed because the mutable reference is still borrowed as immutable
    println!(
        "{:?} {:?} {:?}",
        immutable_ref, mutable_ref, immutable_ref_from_mutable
    );

    // INFO: Assignment of references
    let mut str = String::from("Hello");
    let ref_str_1 = &str; // Immutable reference to str
    let ref_str_2 = ref_str_1; // Another immutable reference to str WARN: (COPY!!!)
    println!("ref_str_1: {}, \nref_str_2: {}", ref_str_1, ref_str_2);

    let ref_str_1 = &mut str;
    let ref_str_2 = ref_str_1;
    // WARN(MOVE): println!("ref_str_1: {}", ref_str_1);
    // WARN(MOVE): ref_str_1 is moved to ref_str_2
    println!("ref_str_2: {ref_str_2}");
}
