use colored::Colorize;
use std::io::{self, Write};

pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading");

    //Visually removes the input from the terminal
    print!("\x1B[1A\x1B[2K");
    io::stdout().flush().unwrap();

    return input.trim_end_matches(&['\r', '\n'][..]).to_string();
}

pub fn write_sentence_with_highlighting(sentence_vec: Vec<char>, input_vec: Vec<char>) {
    for (i, input_char) in input_vec.iter().enumerate() {
        match sentence_vec.get(i) {
            Some(expected_char) if input_char == expected_char => {
                if *input_char == ' ' {
                    print!("{}", "·".green());
                } else {
                    print!("{}", input_char.to_string().green());
                }
            }
            Some(_) | None => {
                if *input_char == ' ' {
                    print!("{}", "·".red());
                } else {
                    print!("{}", input_char.to_string().red());
                }
            }
        }
    }
    println!();
}