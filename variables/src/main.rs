const CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR: u32 = 1 + 1;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR: {CONSTANTS_MUST_BE_UPPER_SNAKE_CASE_TO_AVOID_COMPILE_ERROR}");

    shadowing();

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess: {guess:?}");
}

fn shadowing() {
    let y = 5;

    let y = y + 1; // shadowing allows you to make transformations on a variable, whereafter it is immutable again.

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12
    }

    println!("The value of y is: {y}"); // 6

    let spaces = "    ";
    let spaces = spaces.len(); // shadowing also allows you to change the type of the variable.
    println!("spaces: {spaces:?}");
}
