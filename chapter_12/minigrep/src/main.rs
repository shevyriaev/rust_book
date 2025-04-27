use std::{env, process};

fn main() {
    if let Err(e) = minigrep::run(env::args()) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
