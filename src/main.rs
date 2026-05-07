mod core;

use color_eyre::eyre::Ok;
use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{self, Event, KeyCode};

struct App {
    sentence: String,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        App {
            sentence: core::generate_sentence(3),
            should_quit: false
        }
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('#') => self.should_quit = true,
                KeyCode::Char('r') => {
                    self.sentence = core::generate_sentence(3);
                }
                _ => {}
            }
        }
    }
}

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    let terminal = ratatui::init();
    run(terminal, &mut app)?;
    ratatui::restore();
    Ok(())
}


fn run(mut terminal: DefaultTerminal, app: &mut App) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;
        
        let event = event::read()?;
        app.handle_event(event);

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    use ratatui::widgets::{Paragraph, Block, Borders};
    use ratatui::layout::{Layout, Constraint, Direction};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let para = Paragraph::new(app.sentence.as_str())
        .block(Block::default().borders(Borders::ALL).title("Sentence"));
    frame.render_widget(para, chunks[0]);

    let input_field = Paragraph::new("'r' = new  |  '#' = quit");
    frame.render_widget(input_field, chunks[1]);
}