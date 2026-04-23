use random_word::Lang;

pub fn generate_sentence(length: i32) -> String {
    let mut sentence = String::new();
    for _i in 1..length + 1 {
        let word = &generate_word();
        sentence = sentence + " " + word;
    }
    
    return sentence.trim_start_matches(' ').to_string();
}

pub fn split_sentence(sentence: String) -> Vec<char> {
    return sentence.chars().collect();
}

fn generate_word() -> String {
    return random_word::get(Lang::En).to_string();
}