use std::error::Error;
use std::{env, fs, process};

struct Parameters {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Parameters {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get 'query' parameter"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get 'file_path' parameter"),
        };

        Ok(Parameters {
            query,
            file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line|
            if ignore_case {
                line
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            } else {
                line
                    .contains(&query)
            }
        )
        .collect()
}

pub fn run(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let params =
        Parameters::build(args)
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