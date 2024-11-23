use std::io; // or you could call the function as std::io::stdin in the code without importing here.
use rand::Rng; // Rng is called a trait and must be in scope to use it's methods. Traits are covered more in Chapter 10.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng = local to the current thread of execution. gen_range is a method of the trait Rng.
    
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Here, new is an associated function of the String type. :: means an associated function.
    
        io::stdin()
            .read_line(&mut guess) // &mut means a mutable reference to the guess variable,  which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // first match arm
            Err(_) => continue, // second match arm. _ is a catchall value. The program will ignore the error and continue to the next iteration of the loop.
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // first match arm
            Ordering::Greater => println!("Too big!"), // second match arm
            Ordering::Equal => { // third match arm
                println!("You win!");
                break;
            },
        }
    }
}
