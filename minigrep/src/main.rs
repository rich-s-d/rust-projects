use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args); //debug macro

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){ //The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance. Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value, which would only be ().
        println!("Application error: {e}");
        process::exit(1)
    }
}
