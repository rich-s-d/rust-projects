use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args); //debug macro

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
