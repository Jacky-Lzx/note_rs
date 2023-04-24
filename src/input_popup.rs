use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{App, EditMode, Note};

pub struct InputPopup {}

impl InputPopup {
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
