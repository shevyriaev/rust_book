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

fn rfibonacci(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        n => rfibonacci(n - 1) + rfibonacci(n - 2)
    }
}

fn main() {
    for n in 1..20 + 1 {
        println!("F({n}) = {}\t Fr({n}) = {}", fibonacci(n), rfibonacci(n));
    }
}