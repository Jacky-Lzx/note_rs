use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

/// App holds the state of the application
struct App {
    notes: Vec<String>,
    mode: InputMode,
    input: String,
}
enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            notes: Vec::new(),
            mode: InputMode::Normal,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('a') => app.mode = InputMode::Editing,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    // KeyCode::Char('a') => app.notes.push(String::from("abc")),
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Enter => {
                        app.notes.push(app.input.drain(..).collect());
                        app.input.clear();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());

    let mut texts: Vec<Spans> = Vec::new();

    let mut index = 0;
    for note in &app.notes {
        texts.push(Spans::from(format!("{index}: {note}")));
        index += 1;
    }

    let input_area = Paragraph::new(app.input.as_ref())
        .style(match app.mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_area, chunks[0]);

    let help_message =
        Paragraph::new(texts).block(Block::default().borders(Borders::ALL).title("Notes"));
    f.render_widget(help_message, chunks[1]);

    match app.mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[0].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[0].y + 1,
            )
        }
    }
}
