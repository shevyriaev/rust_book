use std::env::Args;
use std::error::Error;
use std::{env, fs, process};

struct Parameters {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Parameters {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Parameters {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = if ignore_case {
        &query.to_lowercase()
    } else {
        query
    };

    for line in content.lines() {
        let search_line = if ignore_case {
            &line.to_lowercase()
        } else {
            line
        };

        if search_line.contains(&query) {
            result.push(line)
        }
    }

    result
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let params =
        Parameters::build(&args.collect::<Vec<String>>())
            .unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: '{err}'");
                process::exit(1);
            });

    let content = fs::read_to_string(&params.file_path)?;

    let results = if params.ignore_case {
        search(&params.query, &content, true)
    } else {
        search(&params.query, &content, false)
    };

    for line in results {
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
Pick three.
Duct tape";

        assert_eq!(
            vec![
                "safe, fast, productive."
            ],
            search(query, content, false)
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
            search(query, content, true)
        );
    }
}