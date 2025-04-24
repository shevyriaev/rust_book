use std::{io};

fn is_consonant(letter : char) -> bool {
    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => false,
        _ => true
    }
}

fn translate(phrase: &str) -> String {
    let mut result = String::new();

    for word in phrase.to_ascii_lowercase().split_whitespace() {
        let last_char = word.chars().last().unwrap();
        let last_char_is_punctuation = last_char.is_ascii_punctuation();

        for (i, letter) in word.chars().enumerate() {
            if !is_consonant(letter) {
                let last_index = word.chars().count() - if last_char_is_punctuation {1} else {0};

                result.push_str(&word[i..last_index]);
                result.push_str(&word[0..i]);
                result.push_str(&format!("{}", if i == 0 {"yay"} else {"ay"}));

                if last_char_is_punctuation {
                    result.push(last_char);
                }

                result.push(' ');

                break;
            }
        }
    }

    result.trim().to_owned()
}

fn main() {
    println!("Input text(or press <Enter> to use default)");

    let mut phrase = String::new();

    match io::stdin().read_line(&mut phrase) {
        Ok(x) => if x < 3 {
            phrase = String::from(
                "Rust is blazingly fast and memory-efficient: \
                with no runtime or garbage collector, it can power \
                performance-critical services, run on embedded \
                devices, and easily integrate with other languages."
            );
        },
        Err(_) => println!("Error reading line")
    }

    phrase = String::from(phrase.trim());

    println!("[For text]:\t\t\"{}\"", phrase);
    println!("[Translation is]:\t\"{}\"", translate(&phrase));
}