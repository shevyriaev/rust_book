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

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let params =
        Parameters::build(&args.collect::<Vec<String>>())
            .unwrap_or_else(|err| {
                println!("Problem parsing arguments: '{err}'");
                process::exit(1);
            });

    let content = fs::read_to_string(&params.file_path)?;

    // println!(
    //     "Search for '{}' in file '{}'",
    //     &params.query,
    //     &params.file_path,
    // );
    //
    // println!("With text:");
    // for line in content.lines() {
    //     println!("  {line}");
    // }

    for line in search_case_sensitive(&params.query, &content) {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec![
                "safe, fast, productive."
            ],
            search_case_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![
                "Rust:",
                "Trust me."
            ],
            search_case_insensitive(query, content)
        );
    }
}