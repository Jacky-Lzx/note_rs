pub mod input_popup;
mod utils;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use input_popup::InputPopup;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use utils::centered_rect;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// App holds the state of the application
pub struct App {
    notes: Vec<Note>,
    mode: AppMode,
    edit_mode: EditMode,
    input: String,
    input_index: usize,
    current_selection: Option<i32>,
    edit_focus: usize,
    note: Note,
}

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    tag: String,
    command: Vec<String>,
}
#[derive(PartialEq)]
enum AppMode {
    View,
    Editing,
}
#[derive(PartialEq)]
enum EditMode {
    Direct,
    TagInput,
    NoteInput,
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
            mode: AppMode::View,
            edit_mode: EditMode::Direct,
            edit_focus: 0,
            note: Note::new(String::from("")),

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
    fn format<'a>(&self, index: i32, extra_style: Style) -> Spans<'a> {
        // let tag_style = base_style.add
        let tag_style = Style::default().fg(Color::LightBlue).patch(extra_style);
        let ret = Spans::from(vec![
            Span::styled(format!("{}", index), extra_style),
            Span::styled(": ", extra_style),
            Span::styled(format!("{}", self.tag), tag_style),
            Span::styled(" - ", extra_style),
            Span::styled(format!("{:?}", self.command), extra_style),
        ]);
        return ret;
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

fn read_from_file_json() -> Vec<Note> {
    let file = File::open("notes.json").unwrap();
    let p: Vec<Note> = serde_json::from_reader(io::BufReader::new(&file)).unwrap();

    return p;
}
fn write_to_file_json(notes: &Vec<Note>) {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::View => match key.code {
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
                    KeyCode::Char('d') => {
                        // Double check to delete
                        if let Event::Key(key2) = event::read()? {
                            match key2.code {
                                KeyCode::Char('d') => match app.current_selection {
                                    None => {}
                                    Some(index) => {
                                        app.notes.remove(index as usize);
                                        app.current_selection =
                                            Some(std::cmp::min(index, app.notes.len() as i32 - 1));
                                    }
                                },
                                KeyCode::Esc => {}
                                _ => {}
                            }
                        }
                    }

                    KeyCode::Char('a') => {
                        app.mode = AppMode::Editing;
                        app.edit_mode = EditMode::Direct;
                    }
                    _ => {}
                },
                AppMode::Editing => {
                    InputPopup::key_event(&mut app, &key);
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        // .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .constraints([Constraint::Min(1)].as_ref())
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
            let hl_style = Style::default().add_modifier(Modifier::BOLD);
            texts.push(note.format(index, hl_style));
        } else {
            texts.push(note.format(index, Style::default()));
        }
        index += 1;
    }

    // let input_area = Paragraph::new(app.input.as_ref())
    //     .style(match app.mode {
    //         AppMode::View => Style::default(),
    //         AppMode::Editing => Style::default().fg(Color::Yellow),
    //     })
    //     .block(Block::default().borders(Borders::ALL).title("Input"));
    // f.render_widget(input_area, chunks[0]);

    let help_message =
        Paragraph::new(texts).block(Block::default().borders(Borders::ALL).title("Notes"));
    f.render_widget(help_message, chunks[0]);

    match app.mode {
        AppMode::View =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        AppMode::Editing => {
            // let block = Block::default().title("Input").borders(Borders::ALL);
            let area = centered_rect(60, 40, f.size());
            InputPopup::render(f, &area, app);
        }
    }

    match app.mode {
        AppMode::Editing => {}
        _ => {}
    }
}
