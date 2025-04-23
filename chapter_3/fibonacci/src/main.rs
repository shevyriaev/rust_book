fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        n => {
            let mut previous: u32 = 0;
            let mut current: u32 = 1;
            for _ in 2..n {
                let tmp = current;
                current += previous;
                previous = tmp;
            }
            current
        }
    }
}

fn main() {
    for n in 1..20 + 1 {
        println!("F({n}) = {}", fibonacci(n));
    }
}