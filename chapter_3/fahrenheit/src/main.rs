use std::io;

fn main() {
    println!("Input degree in Fahrenheit");

    loop {
        let mut fd = String::new();

        io::stdin()
            .read_line(&mut fd)
            .expect("Failed to read line");

        let fd: f32 =
            match fd.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input valid number");
                    continue;
                }
            };

        println!("{:.2}°F = {:.2}°C", fd, (fd - 32.0) * 5.0 / 9.0);
        break;
    }
}