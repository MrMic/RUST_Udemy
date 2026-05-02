// INFO:
//---------------------------------------------------------------------
//          ╭─────────────────────────────────────────────────────────╮
//          │                     DEREFERENCING:                      │
//          ╰─────────────────────────────────────────────────────────╯
//  - Dereferencing of stack allocated types
//  - Dereferencing of heap allocated types (Box, Rc, Arc)
//
// INFO:
//---------------------------------------------------------------------
//  Borrowing: Sharing access
//  Dereferencing: Accessing the value behind a reference
//---------------------------------------------------------------------

fn main() {
    // Dereferencing of stack allocated types
    let mut some_data = 42;
    let ref_1 = &mut some_data; // ref_1 is a reference to some_data

    let deref_copy = *ref_1; // Dereferencing ref_1  => on the Stack so copy!
    *ref_1 = 13;
    println!("some_data value: {}", some_data); // Output: 13
    println!("deref_copy value: {}", deref_copy); // Output: 42

    /* NOTE:
          ╭─────────────────────────────────────────────────────────╮
          │      Owned types => No & at the start of the type       │
          │         Borrowed => & at the start of the type          │
          ╰─────────────────────────────────────────────────────────╯
    */

    // Dereferencing of heap allocated types (Box, Rc, Arc)
    let mut heap_data = vec![1, 2, 3];
    let ref_1 = &mut heap_data; // ref_1 is a reference to heap_data
    // WARN: let deref_copy = *ref_1; // Dereferencing ref_1 => on the Heap so move!
    // WARN: Ownership move and may be a dangling reference
    ref_1.push(4);
    (*ref_1).push(5); // Dereferencing ref_1 will give us a owned vector, so we can call push on it
    println!("heap_data value: {:?}", heap_data); // Output: [1, 2, 3, 4, 5]
}
