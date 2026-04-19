use random_word::Lang;

fn main() {
    let word = random_word::get(Lang::En);
    println!("{}", word);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error while reading");
    let input = input.trim();

    if input != word {
        println!("'{}' is wrong", input);
    }
    else{
        println!("'{}' is correct", input);   
    }
}
