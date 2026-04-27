mod core;
mod user_interface;

use user_interface::ratatui as ui;

/*
use user_interface::terminal as ui;


fn main() {
    let sentence = core::generate_sentence(10);
    println!("{}", sentence);
    
    let input = ui::get_input();
    ui::write_sentence_with_highlighting(core::split_sentence(sentence), core::split_sentence(input));
}

*/

fn main() {
    let sentence = core::generate_sentence(10);
    ui::init_console().expect("Error while reading");
}