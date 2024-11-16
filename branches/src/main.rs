fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // let x = 1;
    // let y = if x { 0 } else { 1 }; // does not compile because 'falsy' and 'truthy' does not exist in Rust, only true and false.
    // println!("{y}");

    break_in_loop();
    loop_labals_to_disambiguate_between_multiple_loops();
    while_loop();
    for_loop();
    for_loops_are_safer_than_while_loops_and_using_a_range_is_a_common_pattern();
}

fn break_in_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns 20. semicolon optional here.
        }
    };

    println!("The result is {result}");
}

fn loop_labals_to_disambiguate_between_multiple_loops() {
    let mut count = 0;
    'counting_up: loop { // loop labels start with a single quote.
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 { // while is the "loop, if, else, break" pattern.
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loops_are_safer_than_while_loops_and_using_a_range_is_a_common_pattern() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}