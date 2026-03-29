fn main() {
    let my_age = 40;
    println!("My age is {}", my_age);

    let x1 = 40;
    let mut x2 = x1;
    x2 = x1 - 2;
    println!("x1 is {} and x2 is {}", x1, x2); // x2 is 38 because x1 is immutable

    let mut x1 = 40;
    let x2;
    x1 *= 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2);

    let a = "three"; // do not change this line
    let a = 10; // do not change the name of variable 'a'
    println!("a is: {}", a);
}
