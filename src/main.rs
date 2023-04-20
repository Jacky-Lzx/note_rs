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

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// App holds the state of the application
struct App {
    notes: Vec<Note>,
    mode: InputMode,
    input: String,
    input_index: usize,
    current_selection: Option<i32>,
}

use serde::{de::value, Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    tag: String,
    command: Vec<String>,
}
enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> App {
        // Read from file
        let notes_file: Vec<Note> = read_from_file_json();
        read_from_file_json();
        App {
            input: String::new(),
            input_index: 0,
            notes: notes_file,
            mode: InputMode::Normal,
            current_selection: Some(0),
        }
    }
}

impl Note {
    fn new(str_new: String) -> Note {
        let parts: Vec<&str> = str_new.split(":").collect();
        if parts.len() == 1 {
            return Note {
                tag: String::from(""),
                command: vec![String::from(parts[0].trim())],
            };
        } else {
            return Note {
                tag: String::from(parts[0]),
                command: vec![String::from(parts[1].trim())],
            };
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

fn read_from_file() -> Vec<String> {
    let mut notes: Vec<String> = Vec::new();
    // Open the file in read-only mode.
    let file = File::open("notes.txt").unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(note) = line {
            notes.push(note);
        }
    }
    return notes;
}

fn read_from_file_json() -> Vec<Note> {
    // let mut notes: Vec<String> = Vec::new();
    // Open the file in read-only mode.
    let file = File::open("notes.json").unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    // let lines = io::BufReader::new(&file).lines();
    // for line in lines {
    //     if let Ok(note) = line {
    //         notes.push(note);
    //     }
    // }

    let p: Vec<Note> = serde_json::from_reader(io::BufReader::new(&file)).unwrap();

    // println!("{:?}", p);
    return p;
}
fn write_to_file_json(notes: &Vec<Note>) {
    // let mut notes_j: Vec<Note> = Vec::new();
    // for note in notes {
    //     let note_j = Note {
    //         tag: String::from(""),
    //         command: vec![note.clone()],
    //     };
    //
    //     notes_j.push(note_j);
    // }
    let notes_j = notes;
    let j = match serde_json::to_string_pretty(&notes_j) {
        Ok(j) => j,
        Err(why) => panic!("couldn't get json: {}", why),
    };

    let path = Path::new("notes.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(j.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {
            // println!("successfully wrote to {}", display),
        }
    }
}

fn write_to_file(notes: &Vec<String>) {
    let path = Path::new("notes.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    for note in notes {
        let note_write = String::from(note) + "\n";
        match file.write_all(note_write.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {
                // println!("successfully wrote to {}", display),
            }
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        // write_to_file(&app.notes);
                        write_to_file_json(&app.notes);
                        return Ok(());
                    }
                    KeyCode::Char('e') => {
                        use std::process::Command;

                        Command::new("nvim")
                            .arg("/tmp/note_rs.tmp")
                            .status()
                            .expect("failed to execute process");

                        if std::path::Path::new("/tmp/note_rs.tmp").exists() {
                            // Open the file in read-only mode.
                            let file = File::open("/tmp/note_rs.tmp").unwrap();
                            // Read the file line by line, and return an iterator of the lines of the file.
                            let lines = io::BufReader::new(file).lines();
                            for line in lines {
                                if let Ok(note) = line {
                                    // app.notes.push(Note {
                                    //     tag: String::from(""),
                                    //     command: vec![note],
                                    // });
                                    app.notes.push(Note::new(note));
                                    break;
                                }
                            }

                            use std::fs;
                            fs::remove_file("/tmp/note_rs.tmp")?;
                        }

                        // let mut stdout = io::stdout();
                        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

                        terminal.clear()?;
                    }
                    KeyCode::Char('j') => {
                        app.current_selection = match app.current_selection {
                            None => None,
                            Some(i) => Some(std::cmp::min(app.notes.len() as i32 - 1, i + 1)),
                        };
                    }
                    KeyCode::Char('k') => {
                        app.current_selection = match app.current_selection {
                            None => None,
                            Some(i) => Some(std::cmp::max(0, i - 1)),
                        };
                    }
                    KeyCode::Char('d') => match app.current_selection {
                        None => {}
                        Some(index) => {
                            app.notes.remove(index as usize);
                            app.current_selection = Some(std::cmp::max(index - 1, 0));
                        }
                    },

                    KeyCode::Char('a') => {
                        app.mode = InputMode::Editing;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    KeyCode::Left => {
                        app.input_index -= 1;
                        app.input_index = std::cmp::max(0, app.input_index);
                    }
                    KeyCode::Right => {
                        app.input_index += 1;
                        app.input_index = std::cmp::min(app.input.len(), app.input_index);
                    }
                    // KeyCode::Char('a') => app.notes.push(String::from("abc")),
                    KeyCode::Char(c) => {
                        app.input.insert(app.input_index, c);
                        app.input_index += 1;
                    }
                    KeyCode::Enter => {
                        let note: String = app.input.drain(..).collect();
                        if note.len() != 0 {
                            // app.notes.push(Note {
                            //     tag: String::from(""),
                            //     command: vec![note],
                            // });
                            app.notes.push(Note::new(note));
                        }
                        app.input.clear();
                        app.mode = InputMode::Normal;
                        app.input_index = 0;
                    }
                    KeyCode::Backspace => {
                        // app.input.pop();
                        if app.input_index > 0 {
                            app.input.remove(app.input_index - 1);
                            app.input_index -= 1;
                            app.input_index = std::cmp::max(0, app.input_index);
                        }
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

    let mut index: i32 = 0;
    for note in &app.notes {
        if index
            == match app.current_selection {
                None => -1,
                Some(value) => value,
            }
        {
            let style = Style::default().fg(Color::LightBlue).bg(Color::Gray);
            let hl_style = Style::default().bg(Color::Gray);
            texts.push(Spans::from(vec![
                Span::styled(format!("{}", index), hl_style),
                Span::styled(": ", hl_style),
                Span::styled(format!("{}", note.tag), style),
                Span::styled(" - ", hl_style),
                Span::styled(format!("{:?}", note.command), hl_style),
            ]));
        } else {
            let style = Style::default().fg(Color::LightBlue);
            texts.push(Spans::from(vec![
                Span::raw(format!("{}", index)),
                Span::raw(": "),
                Span::styled(format!("{}", note.tag), style),
                Span::raw(" - "),
                Span::raw(format!("{:?}", note.command)),
            ]));
            // texts.push(Spans::from(format!(
            //     "{}: {} - {:?}",
            //     index, note.tag, note.command
            // )));
        }
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
                // chunks[0].x + app.input.width() as u16 + 1,
                chunks[0].x + app.input_index as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[0].y + 1,
            )
        }
    }
}
