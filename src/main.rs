use random_word::Lang;

fn main() {
    let mut sentence = String::new();
    for _i in 1..4{
        let word = random_word::get(Lang::En);
        sentence = sentence + " " + word;
    }
    let sentence = sentence.trim_start_matches(' ');
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
