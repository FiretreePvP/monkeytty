use random_word::Lang;

fn main() {
    let sentence = generate_sentence(5);
    println!("{}", sentence);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading");
    let input = input.trim_end_matches(&['\r', '\n'][..]);

    if input != sentence {
        println!("'{}' is wrong", input);
    }
    else{
        println!("'{}' is correct", input);
    }
}

fn generate_sentence(length: i32) -> String{
    let mut sentence = String::new();
    for _i in 1..length + 1{
        let word = random_word::get(Lang::En);
        sentence = sentence + " " + word;
    }
    
    return sentence.trim_start_matches(' ').to_string();
}