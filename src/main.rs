// pub mod input_popup;

mod note;
mod parser;
mod utils;

use std::cell::RefCell;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;

use parser::Parser;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
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
    current_note_index: usize,
    edit_focus: usize,
}

#[derive(PartialEq)]
enum AppMode {
    View,
    Editing,
    Exit,
}
#[derive(PartialEq)]
enum EditMode {
    Direct,
    TagInput,
    NoteInput,
}

fn find_note_index(notes: &Vec<RefNote>, note: &RefNote) -> usize {
    notes
        .iter()
        .position(|x| Rc::ptr_eq(x, note))
        .unwrap_or_default()
}

impl Default for App {
    fn default() -> App {
        // Read from file
        let notes_file: Vec<RefNote> = Parser::from_markdown();
        App {
            input: String::new(),
            input_index: 0,
            notes: notes_file,
            mode: AppMode::View,
            edit_mode: EditMode::Direct,
            edit_focus: 0,
            current_selection: Some(0),
            current_note_index: 1,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.mode != AppMode::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
}

impl App {
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            AppMode::View => match key_event.code {
                KeyCode::Char('q') => {
                    // write_to_file(&app.notes);
                    // write_to_file_json(&app.notes);
                    self.mode = AppMode::Exit;
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
                                // app.notes.push(Note::new());
                                break;
                            }
                        }

                        use std::fs;
                        fs::remove_file("/tmp/note_rs.tmp").unwrap();
                    }

                    // let mut stdout = io::stdout();
                    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

                    // terminal.clear();
                }
                KeyCode::Char('h') => {
                    let parent_index = find_note_index(
                        &self.notes,
                        self.notes[self.current_note_index]
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap(),
                    );
                    self.current_note_index = if parent_index == 0 {
                        self.current_note_index
                    } else {
                        parent_index
                    };
                }
                KeyCode::Char('l') => {
                    let child_index = find_note_index(
                        &self.notes,
                        if let Some(note) = self.notes[self.current_note_index]
                            .borrow()
                            .children
                            .first()
                        {
                            note.as_ref().unwrap()
                        } else {
                            &self.notes[self.current_note_index]
                        },
                    );
                    self.current_note_index = child_index;
                }
                KeyCode::Char('j') => {
                    let parent_index = find_note_index(
                        &self.notes,
                        self.notes[self.current_note_index]
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap(),
                    );
                    let next_sibling_index = find_note_index(
                        &self.notes,
                        self.notes
                            .iter()
                            .enumerate()
                            .filter(|(index, x)| {
                                index > &self.current_note_index
                                    && Rc::ptr_eq(
                                        x.borrow().parent.as_ref().unwrap(),
                                        &self.notes[parent_index],
                                    )
                            })
                            .next()
                            .unwrap_or((0, &self.notes[self.current_note_index]))
                            .1,
                    );
                    let max_sibling_index = find_note_index(
                        &self.notes,
                        self.notes[parent_index]
                            .borrow()
                            .children
                            .last()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    );
                    self.current_note_index = min(next_sibling_index, max_sibling_index);
                }
                KeyCode::Char('k') => {
                    let parent_index = find_note_index(
                        &self.notes,
                        self.notes[self.current_note_index]
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap(),
                    );
                    let null_ref = Rc::new(RefCell::new(Note::new()));
                    let prev_sibling_index = find_note_index(
                        &self.notes,
                        self.notes
                            .iter()
                            .enumerate()
                            .filter(|(index, x)| {
                                index < &self.current_note_index
                                    && Rc::ptr_eq(
                                        x.borrow().parent.as_ref().unwrap_or_else(|| &null_ref),
                                        &self.notes[parent_index],
                                    )
                            })
                            .last()
                            .unwrap_or((0, &self.notes[self.current_note_index]))
                            .1,
                    );
                    let min_sibling_index = find_note_index(
                        &self.notes,
                        self.notes[parent_index]
                            .borrow()
                            .children
                            .first()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    );
                    self.current_note_index = max(prev_sibling_index, min_sibling_index);
                }
                KeyCode::Char('d') => {
                    // Double check to delete
                    if let Event::Key(key2) = event::read().unwrap() {
                        match key2.code {
                            KeyCode::Char('d') => match self.current_selection {
                                None => {}
                                Some(index) => {
                                    self.notes.remove(index as usize);
                                    self.current_selection =
                                        Some(std::cmp::min(index, self.notes.len() as i32 - 1));
                                }
                            },
                            KeyCode::Esc => {}
                            _ => {}
                        }
                    }
                }

                KeyCode::Char('a') => {
                    self.mode = AppMode::Editing;
                    self.edit_mode = EditMode::Direct;
                }
                _ => {}
            },
            AppMode::Editing => {
                // InputPopup::key_event(&mut app, &key);
            }
            AppMode::Exit => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints(
                [
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ]
                .as_ref(),
            )
            .split(frame.area());

        const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

        for depth in 2..4 {
            // Create a list for the notes with depth
            let notes_at_depth: Vec<ListItem> = self
                .notes
                .iter()
                .filter(|n| n.borrow().depth == depth)
                .map(|note| {
                    if Rc::ptr_eq(note, &self.notes[self.current_note_index]) {
                        ListItem::from(note.borrow().title.clone()).style(SELECTED_STYLE)
                    } else {
                        ListItem::from(note.borrow().title.clone())
                    }
                })
                .collect();

            let list = List::new(notes_at_depth).block(
                Block::new()
                    .title(format!("Depth: {}", depth))
                    .borders(Borders::ALL),
            );
            frame.render_widget(list, layouts[depth - 2]);
        }
        // Render the note
        let para = Paragraph::new(self.notes[self.current_note_index].borrow().content.clone())
            .block(Block::default().borders(Borders::ALL).title("Note"));
        frame.render_widget(para, layouts[2]);
    }
}
