use std::{env, fs, process};

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
            file_path: args[2].clone()
        })
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let params = Parameters::build(&args)
        .unwrap_or_else(|err|{
            println!("Problem parsing arguments: '{err}'");
            process::exit(1);
        });

    println!("Search for '{}' in file '{}' with text:", params.query, params.file_path);

    let content = fs::read_to_string(params.file_path)
        .expect("Should have been able to read the file");

    println!("{content}");
}
