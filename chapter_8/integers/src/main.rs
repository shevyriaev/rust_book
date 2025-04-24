use std::collections::{HashMap};
use std::{io};
use rand::{Rng};

fn median(vector:&Vec<i32>) -> Option<i32> {
    if vector.len() == 0 {
        return None
    };

    let mut v: Vec<i32> = vector.clone();

    v.sort();

    Some(v[v.len() / 2])
}

fn mode(vector: &Vec<i32>) -> Option<i32> {
    if vector.len() == 0 {
        return None
    };

    let mut counters = HashMap::new();

    for i in vector {
        let count = counters.entry(i).or_insert(0);
        *count += 1;
    }

    let result = match counters.iter().max_by_key(|&(_, v)| v) {
        None => vector[0],
        Some((&k, _)) => *k
    };

    Some(result)
}

fn main() {
    let mut integers: Vec<i32> = Vec::new();

    println!("Input vector size:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Filed to read line");

    match input.trim().parse() as Result<u32, _> {
        Ok(x) => {
            for _ in 0..x-1 {
                let new_integer = rand::rng().random_range(1..101);
                integers.push(new_integer);
            };
        },
        Err(e) => {
            println!("{e}");
            return;
        }
    }

    println!("{integers:?}");

    match median(&integers) {
        Option::None => println!("median = none"),
        Option::Some(x) => println!("median = {x}")
    };

    match mode(&integers) {
        Option::None => println!("mode = none"),
        Option::Some(x) => println!("mode = {x}")
    };
}
