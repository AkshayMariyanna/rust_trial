use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguements: {}", err);
    //     process::exit(1);
    // });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
