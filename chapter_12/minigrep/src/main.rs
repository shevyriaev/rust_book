use std::{env, process};

fn main() {
    if let Err(e) = minigrep::run(env::args()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
