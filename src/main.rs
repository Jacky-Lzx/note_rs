// pub mod input_popup;
mod note;
mod parser;
mod utils;

use std::fs::File;
use std::io::{self, BufRead};

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

use note::Note;
use note::RefNote;
/// App holds the state of the application
pub struct App {
    notes: Vec<RefNote>,
    mode: AppMode,
    edit_mode: EditMode,
    input: String,
    input_index: usize,
    current_selection: Option<i32>,
    edit_focus: usize,
    note: Note,
    exit: bool,
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
        let notes_file: Vec<RefNote> = read_from_markdown();
        App {
            input: String::new(),
            input_index: 0,
            notes: notes_file,
            mode: AppMode::View,
            edit_mode: EditMode::Direct,
            edit_focus: 0,
            // note: Note::from(String::from(""), String::from(""), 0),
            note: Note::new(),
            current_selection: Some(0),
            exit: false,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;

            // if let Event::Key(key) = event::read()? {
            //     match app.mode {
            //         AppMode::View => match key.code {
            //             KeyCode::Char('q') => {
            //                 // write_to_file(&app.notes);
            //                 // write_to_file_json(&app.notes);
            //                 return Ok(());
            //             }
            //             KeyCode::Char('e') => {
            //                 use std::process::Command;
            //
            //                 Command::new("nvim")
            //                     .arg("/tmp/note_rs.tmp")
            //                     .status()
            //                     .expect("failed to execute process");
            //
            //                 if std::path::Path::new("/tmp/note_rs.tmp").exists() {
            //                     // Open the file in read-only mode.
            //                     let file = File::open("/tmp/note_rs.tmp").unwrap();
            //                     // Read the file line by line, and return an iterator of the lines of the file.
            //                     let lines = io::BufReader::new(file).lines();
            //                     for line in lines {
            //                         if let Ok(note) = line {
            //                             // app.notes.push(Note {
            //                             //     tag: String::from(""),
            //                             //     command: vec![note],
            //                             // });
            //                             // app.notes.push(Note::new());
            //                             break;
            //                         }
            //                     }
            //
            //                     use std::fs;
            //                     fs::remove_file("/tmp/note_rs.tmp")?;
            //                 }
            //
            //                 // let mut stdout = io::stdout();
            //                 // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            //
            //                 terminal.clear()?;
            //             }
            //             KeyCode::Char('j') => {
            //                 app.current_selection = match app.current_selection {
            //                     None => None,
            //                     Some(i) => Some(std::cmp::min(app.notes.len() as i32 - 1, i + 1)),
            //                 };
            //             }
            //             KeyCode::Char('k') => {
            //                 app.current_selection = match app.current_selection {
            //                     None => None,
            //                     Some(i) => Some(std::cmp::max(0, i - 1)),
            //                 };
            //             }
            //             KeyCode::Char('d') => {
            //                 // Double check to delete
            //                 if let Event::Key(key2) = event::read()? {
            //                     match key2.code {
            //                         KeyCode::Char('d') => match app.current_selection {
            //                             None => {}
            //                             Some(index) => {
            //                                 app.notes.remove(index as usize);
            //                                 app.current_selection = Some(std::cmp::min(
            //                                     index,
            //                                     app.notes.len() as i32 - 1,
            //                                 ));
            //                             }
            //                         },
            //                         KeyCode::Esc => {}
            //                         _ => {}
            //                     }
            //                 }
            //             }
            //
            //             KeyCode::Char('a') => {
            //                 app.mode = AppMode::Editing;
            //                 app.edit_mode = EditMode::Direct;
            //             }
            //             _ => {}
            //         },
            //         AppMode::Editing => {
            //             // InputPopup::key_event(&mut app, &key);
            //         }
            //     }
            // }
        }
        Ok(())
    }
}

use parser::Parser;
fn read_from_markdown() -> Vec<RefNote> {
    // Read from file "Note.md"
    let file = File::open("Note.md").unwrap();
    // Store all lines in file to a vector
    let lines: Vec<String> = io::BufReader::new(&file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    Parser::parse(lines)
}

impl App {
    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
    fn ui(&self, frame: &mut Frame) {
        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(2)
        //     // .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        //     .constraints([Constraint::Min(1)].as_ref())
        //     .split(frame.size());
        //
        // let mut texts: Vec<Spans> = Vec::new();
        //
        // let mut index: i32 = 0;
        // for note in &app.notes {
        //     if index
        //         == match app.current_selection {
        //             None => -1,
        //             Some(value) => value,
        //         }
        //     {
        //         let hl_style = Style::default().add_modifier(Modifier::BOLD);
        //         texts.push(note.borrow().format(index, hl_style));
        //     } else {
        //         texts.push(note.borrow().format(index, Style::default()));
        //     }
        //     index += 1;
        // }
        //
        // // let input_area = Paragraph::new(app.input.as_ref())
        // //     .style(match app.mode {
        // //         AppMode::View => Style::default(),
        // //         AppMode::Editing => Style::default().fg(Color::Yellow),
        // //     })
        // //     .block(Block::default().borders(Borders::ALL).title("Input"));
        // // f.render_widget(input_area, chunks[0]);
        //
        // let help_message =
        //     Paragraph::new(texts).block(Block::default().borders(Borders::ALL).title("Notes"));
        // frame.render_widget(help_message, chunks[0]);
        //
        // match app.mode {
        //     AppMode::View =>
        //         // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        //         {}
        //
        //     AppMode::Editing => {
        //         // let block = Block::default().title("Input").borders(Borders::ALL);
        //         let area = centered_rect(60, 40, frame.size());
        //         // InputPopup::render(f, &area, app);
        //     }
        // }
        //
        // match app.mode {
        //     AppMode::Editing => {}
        //     _ => {}
        // }
    }
}
