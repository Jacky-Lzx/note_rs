use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{App, AppMode, EditMode, Note};

pub struct InputPopup {}

impl InputPopup {
    pub fn key_event(app: &mut App, key: &KeyEvent) {
        match app.edit_mode {
            EditMode::NoteInput => {
                match key.code {
                    KeyCode::Esc => {
                        // app.mode = AppMode::Editing;
                        app.edit_mode = EditMode::Direct;
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
                            // app.notes.push(Note::new(note));
                            app.note.command = vec![note];
                        }
                        app.input.clear();
                        // app.mode = AppMode::View;
                        app.edit_mode = EditMode::Direct;
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
                }
            }
            EditMode::Direct => match key.code {
                KeyCode::Esc => {
                    app.mode = AppMode::View;
                }
                KeyCode::Char('j') => {
                    // app.edit_focus += 1;
                    app.edit_focus = std::cmp::min(1, app.edit_focus + 1);
                }
                KeyCode::Char('k') => {
                    if app.edit_focus != 0 {
                        app.edit_focus = std::cmp::max(0, app.edit_focus - 1);
                    }
                }
                KeyCode::Char('i') => {
                    if app.edit_focus == 0 {
                        app.edit_mode = EditMode::TagInput;
                        app.input = String::from(&app.note.tag);
                        app.input_index = app.input.len();
                    } else if app.edit_focus == 1 {
                        app.edit_mode = EditMode::NoteInput;
                        app.input = String::from(&app.note.command[0]);
                        app.input_index = app.input.len();
                    }
                }
                KeyCode::Enter => {
                    app.notes.push(Note {
                        tag: String::from(&app.note.tag),
                        command: vec![String::from(&app.note.command[0])],
                    });
                    app.note = Note {
                        tag: String::from(""),
                        command: vec![String::from("")],
                    };
                    app.edit_mode = EditMode::Direct;
                    app.mode = AppMode::View;
                }
                _ => {}
            },
            EditMode::TagInput => match key.code {
                KeyCode::Esc => {
                    // app.mode = AppMode::Editing;
                    app.edit_mode = EditMode::Direct;
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
                        // app.notes.push(Note::new(note));
                        app.note.tag = note;
                    }
                    app.input.clear();
                    // app.mode = AppMode::View;
                    app.edit_mode = EditMode::Direct;
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
    pub fn render<B: Backend>(f: &mut Frame<B>, area: &Rect, app: &App) {
        let input_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(*area);
        // let mut tag_block = Block::default().title("Tag").borders(Borders::ALL);
        let mut tag_text: &str = app.note.tag.as_ref();
        let focus_style = Style::default().fg(Color::Yellow);
        let editing_style = Style::default().fg(Color::Cyan);
        let mut tag_style = Style::default();
        if app.edit_focus == 0 {
            if app.edit_mode == EditMode::TagInput {
                tag_text = app.input.as_ref();
                tag_style = tag_style.patch(editing_style);
            } else {
                tag_style = tag_style.patch(focus_style);
            }
        }
        let tag_box = Paragraph::new(tag_text)
            .block(Block::default().title("Tag").borders(Borders::ALL))
            .style(tag_style);

        let mut note_text: &str = app.note.command[0].as_ref();
        let mut note_style = Style::default();

        if app.edit_focus == 1 {
            if app.edit_mode == EditMode::NoteInput {
                note_text = app.input.as_ref();
                note_style = note_style.patch(editing_style);
            } else {
                note_style = note_style.patch(focus_style);
            }
        }

        let note_box = Paragraph::new(note_text)
            .block(Block::default().title("Command").borders(Borders::ALL))
            .style(note_style);

        f.render_widget(Clear, *area); //this clears out the background
        f.render_widget(tag_box, input_chunks[0]);
        f.render_widget(note_box, input_chunks[1]);

        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        match app.edit_mode {
            EditMode::Direct => {}
            _ => {
                f.set_cursor(
                    input_chunks[app.edit_focus].x + app.input_index as u16 + 1,
                    // Move one line down, from the border to the input line
                    input_chunks[app.edit_focus].y + 1,
                )
            }
        }
    }
}
