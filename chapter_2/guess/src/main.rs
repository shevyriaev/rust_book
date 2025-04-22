use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn print_incorrect_guess() {
    println!("==> Please input number between 1 and 100");
}

fn main() {
    println!("==> Guess number between 1 and 100");
    let secret_number: u32 = rand::rng().random_range(1..100);

    loop {
        println!("==> Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => {
                if x < 1 || x > 100 {
                    print_incorrect_guess();
                    continue;
                }
                x
            }
            Err(_) => {
                print_incorrect_guess();
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("==> {guess} is too small!"),
            Ordering::Greater => println!("==> {guess} is too big!"),
            _ => {
                println!("[ You win!!! ]");
                break;
            }
        }
    }
}
