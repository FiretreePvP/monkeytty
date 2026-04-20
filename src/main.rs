use random_word::Lang;
use colored::Colorize;

fn main() {
    let sentence = generate_sentence(2);
    println!("{}", sentence);

    let input = get_input();
    let result = input_correct(input, sentence);

    println!("{}", if result {"Correct!".green()} else {"Incorrect!".red()})
}

fn generate_sentence(length: i32) -> String {
    let mut sentence = String::new();
    for _i in 1..length + 1 {
        let word = random_word::get(Lang::En);
        sentence = sentence + " " + word;
    }
    
    return sentence.trim_start_matches(' ').to_string();
}

fn input_correct(input: String, sentence: String) -> bool {
    if input != sentence {
        return false;
    }
    return true;
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading");
    return input.trim_end_matches(&['\r', '\n'][..]).to_string();
}