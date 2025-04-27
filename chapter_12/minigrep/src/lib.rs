use std::env::Args;
use std::error::Error;
use std::{fs, process};

struct Parameters {
    query: String,
    file_path: String,
}

impl Parameters {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Parameters {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let params =
        Parameters::build(&args.collect::<Vec<String>>())
            .unwrap_or_else(|err| {
                println!("Problem parsing arguments: '{err}'");
                process::exit(1);
            });

    let content = fs::read_to_string(&params.file_path)?;

    println!(
        "Search for '{}' in file '{}'",
        &params.query,
        &params.file_path,
    );

    println!("With text:");
    for line in content.lines() {
        println!("  {line}");
    }

    Ok(())
}