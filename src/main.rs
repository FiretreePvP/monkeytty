use random_word::Lang;
use colored::Colorize;

fn main() {
    let sentence = generate_sentence(2);
    println!("{}", sentence);

    let input = get_input();

    write_sentence_with_highlighting(split_sentence(sentence), split_sentence(input));
}

fn generate_sentence(length: i32) -> String {
    let mut sentence = String::new();
    for _i in 1..length + 1 {
        let word = random_word::get(Lang::En);
        sentence = sentence + " " + word;
    }
    
    return sentence.trim_start_matches(' ').to_string();
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading");
    return input.trim_end_matches(&['\r', '\n'][..]).to_string();
}

fn split_sentence(sentence: String) -> Vec<char> {
    return sentence.chars().collect();
}

fn write_sentence_with_highlighting(sentence_vec: Vec<char>, input_vec: Vec<char>) {
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