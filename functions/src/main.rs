fn main() {
    another_function(5);
    let z = five();
    println!("The value of z is: {z}");


    println!("{}", f({ // here, {} provides syntactic scope for a let bindings returning an expression that evaluates to 2, so the return value from f is 3.
        let y = 1;
        y + 1
      }));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// functions return implicity the last expression they contain.
fn five() -> i32 {
    5// return values are expressions, and expressions don't have semi colons.
}

fn f(x: i32) -> i32 { x + 1 }

/* multi line comment syntax
is very handy.
*/
