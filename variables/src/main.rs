const CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR: u32 = 1 + 1;

fn main() {
    let mut x = 5; // variables are immutable wihtout the mut keyword.
    println!("The value of x is {x}");
    x = 6; // this would not complile if the keyword mut was not used to declare the variable as you cannot assign a value to immutable variables.
    println!("The value of x is {x}");
    println!("CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR: {CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR}");

    shadowing();

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess: {guess:?}");

    let floating_point_32: f32 = 1.0 / 2.0;
    let floating_point: f64= 1.0 / 2.0;
    println!("floating_point: {floating_point:?}");
    println!("floating_point_32: {floating_point_32:?}");

    let t = true;
    let t: bool = false;

    // compound types
    // tuples - have a fixed length
    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, false);
    let (x, y, z,  a) = tup;
    let y = tup.1;
    let z = tup.3;

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("x: {x:?}");

    // arrays - have a fixed length in Rust. Vectors can have a varying length.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


}

fn shadowing() {
    let y = 5;

    let y = y + 1; // shadowing allows you to make transformations on a variable, whereafter it is immutable again.

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }

    println!("The value of y is: {y}"); // 6

    let spaces: &str = "    ";
    let spaces: usize = spaces.len(); // shadowing also allows you to change the type of the variable.
    println!("spaces: {spaces:?}");
}
