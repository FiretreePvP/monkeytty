mod core;
mod ui;

fn main() {
    let sentence = core::generate_sentence(10);
    println!("{}", sentence);
    
    let input = ui::get_input();
    ui::write_sentence_with_highlighting(core::split_sentence(sentence), core::split_sentence(input));
}