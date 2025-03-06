use std::cell::RefCell;
use std::rc::Rc;

// use tui::style::{Color, Style};
// use tui::text::{Span, Spans};

type Link = Option<Rc<RefCell<Note>>>;
pub type RefNote = Rc<RefCell<Note>>;

#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub depth: usize,
    pub children: Vec<Link>,
    pub parent: Link,
}

impl Note {
    pub fn from(title: String, content: String, depth: usize) -> RefNote {
        Rc::new(RefCell::new(Note {
            title,
            content,
            depth,
            children: vec![],
            parent: None,
        }))
    }
    pub fn new() -> Note {
        Note {
            title: "".to_string(),
            content: "".to_string(),
            depth: 0,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(parent: RefNote, child: RefNote) {
        parent.borrow_mut().children.push(Some(child.clone()));
        child.borrow_mut().parent = Some(parent.clone());
    }

    #[allow(dead_code)]
    pub fn print_children(&self) {
        println!("{}Title: {}", "  ".repeat(self.depth), self.title);
        println!(
            "{}Content: {}",
            "  ".repeat(self.depth + 1),
            self.content.replace("\n", "|")
        );
        for child in self.children.iter() {
            if let Some(child) = child {
                child.borrow().print_children();
            }
        }
    }

    // pub fn format<'a>(&self, index: i32, extra_style: Style) -> Spans<'a> {
    // let tag_style = base_style.add
    // let title_style = Style::default().fg(Color::LightBlue).patch(extra_style);
    // let ret = Spans::from(vec![
    //     Span::styled(format!("{}", index), extra_style),
    //     Span::styled(": ", extra_style),
    //     Span::styled(format!("{}", self.title), title_style),
    //     Span::styled(" - ", extra_style),
    //     Span::styled(format!("{:?}", self.content), extra_style),
    // ]);
    // return ret;
    // }
}
