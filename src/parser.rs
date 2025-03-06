use crate::note::Note;
use crate::note::RefNote;
pub struct Parser {}

impl Parser {
    fn solve_relationship(notes: &Vec<RefNote>) {
        let length = notes.len();
        if length == 0 {
            return;
        }

        let mut stack: Vec<usize> = Vec::new();

        for i in 0..notes.len() {
            if i == 0 {
                stack.push(i);
                continue;
            }
            let mut parent_index = stack.pop().unwrap();
            while notes[parent_index].borrow().depth >= notes[i].borrow().depth {
                parent_index = stack.pop().unwrap();
            }
            Note::add_child(notes[parent_index].clone(), notes[i].clone());
            stack.push(parent_index);
            stack.push(i);
        }
    }

    pub fn parse(lines: Vec<String>) -> Vec<RefNote> {
        // Parse the input
        let mut notes: Vec<RefNote> = Vec::new();

        let mut start = 0;
        for (index, l) in lines.iter().enumerate() {
            if index == 0 {
                continue;
            }
            // println!("{}", l);
            // println!();
            if l.starts_with("#") {
                let new_note = Self::parse_a_note(&lines, start, index);
                notes.push(new_note);
                start = index;
            }
        }
        let note = Self::parse_a_note(&lines, start, lines.len());
        notes.push(note);

        // Solve the parent-child relationship
        Self::solve_relationship(&notes);

        // notes[0].borrow().print_children();
        // println!();

        notes
    }

    // start: inclusive, end: exclusive
    fn parse_a_note(lines: &Vec<String>, start: usize, end: usize) -> RefNote {
        let l = lines[start].clone();

        // Count the number of #s
        let mut count = 0;
        for c in l.chars() {
            if c == '#' {
                count += 1;
            } else {
                break;
            }
        }

        let title = lines[start][count..].trim_start().to_string();
        let content = if start == end {
            String::from("")
        } else {
            lines[start + 1..end].join("\n")
        };

        Note::from(title, content, count)
    }
}
