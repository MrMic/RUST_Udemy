#![allow(unused_variables)]

/*
    - Variables Convention
        - Variables should be in snake_case, which means all lowercase letters with underscores separating words.
        - Variables should not start with a number or contain special characters (except for underscores).
        - Variables should not be reserved keywords in Rust (e.g., let, fn, if, else, etc.).
    - Unused Variables Convention
        - If a variable is declared but not used, it should be prefixed with an underscore (_) to indicate that it is intentionally unused.
    - Statics
*/

fn main() {
    // - Variable Convention
    // let x = 0;
    let _x = 10_000;

    // Statics are similar to constants
    /*
      INFO:
      const — compile-time value, inlined everywhere
      const PI: f32 = 3.14159;
      - Value is inlined at every use site by the compiler (like a #define in C)
      - No memory address — it doesn't exist at runtime as a variable
      - Can be used in const contexts (array sizes, match arms, generic params)
      - Zero runtime overhead

        INFO:
      static — a fixed memory location
      static MAX_POINTS: u32 = 100_000;
      - Lives at a fixed memory address for the entire program lifetime
      - Has a real address you can take (&MAX_POINTS)
      - Required for static mut (mutable global state — needs unsafe)
      - Needed for FFI or when a stable address is required

      static is the right choice only when you specifically need:
          - A stable memory address
          - Global shared state (prefer OnceLock or LazyLock over static mut)
          - Large data (e.g., a static byte array) you don't want copied everywhere
    */

    // static MAX_POINTS: u32 = 100_000;
    static _MAX_POINTS: u32 = 100_000;
    const PI: f32 = std::f32::consts::PI;

    let a = PI;
    let b = PI;
}
