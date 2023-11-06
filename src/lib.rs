pub mod tiny;

// WARN: spaghetti code
// TODO: create a new branch, use a different implementation

// TODO: attach a cursor to the current buffer somehow
#[derive(Debug)]
pub struct Cursor {
    pub caret: char,
    position: (u32, u32), // zero-indexed (x, y)
}

impl Cursor {
    pub fn new(caret: char) -> Cursor {
        Cursor {
            caret,
            position: (0, 0),
        }
    }

    fn get_pos(&self) -> (u32, u32) {
        self.position
    }

    // fn print_pos(&self) where{
    //     let mut cp = String::new(); // line showing  cursor position with caret
    //     let p = self.position;
    //     for _ in 0..p {
    //         cp.push_str(" ");
    //     }
    //     cp.push_str(stringify!(cs));
    // }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub text: String,
    has_focus: bool,
}

impl Line {
    pub fn new(text: String, focused: bool) -> Line {
        Line {
            text,
            has_focus: focused,
        }
    }

    pub fn empty(focused: bool) -> Line {
        Line {
            text: String::from(""),
            has_focus: focused,
        }
    }

    fn has_focus(&self) -> bool {
        self.has_focus
    }

    // fn print_line(&self) {
    //     if let Some(cs) = &self.cursor {
    //         cs.print_pos();
    //         println!("{}", self.text);
    //         cs.print_pos();
    //     } else {
    //         println!("{}", self.text);
    //     }
    // }
}

// use Builder pattern
// edit buffer that holds text
/// The Buffer view is such:
/// L-L-L-L-...-*
/// where L is the text at a line, and - denotes the line ending byte,
/// with * denoting multiples are allowed
// TODO: define errors
#[derive(Debug)]
pub struct Buffer {
    filename: String,
    pub lines: Vec<Line>,
    // pub cursor: Cursor,
    line_count: usize,
}

impl Buffer {
    pub fn empty() -> Buffer {
        Buffer {
            filename: String::from("EMPTY"),
            lines: Vec::new(),
            // cursor: Cursor::new('^'),
            line_count: 0,
        }
    }

    pub fn init(filename: &str, text: String) -> Buffer {
        let mut lines = Vec::new();

        // text is empty string?
        if !text.is_empty() {
            for (i, line) in text.split_terminator('\n').enumerate() {
                let mut has_focus = false;
                if i == 0 {
                    has_focus = true;
                }
                lines.push(Line {
                    text: line.to_owned(),
                    has_focus,
                });
            }
        }

        let line_count = lines.len();
        Buffer {
            filename: filename.to_owned(),
            lines,
            // cursor: Cursor::new('^'),
            line_count,
        }
    }
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_owned();
    }

    // TODO: update cursor position
    // TODO: move the cursor within the line
    // but for now just receive focus
    pub fn goto_nth_line(&mut self, n: u32) -> bool {
        for (i, line) in self.lines.iter_mut().enumerate() {
            if i == (n as usize) {
                line.has_focus = true;
                return true;
            }
        }
        false
    }

    // pub fn get_cursor_pos_x(&self) -> u32 {
    //     self.cursor.get_pos().0
    // }

    // pub fn get_cursor_pos_y(&self) -> u32 {
    //     self.cursor.get_pos().1
    // }

    // pub fn goto_next_line(&mut self) -> bool {
    //     // let line_no = self.get_cursor_pos_y();
    //     if (line_no + 1) as usize > self.line_count {
    //         panic!("Going past infinity is for Buzz only");
    //     } else {
    //         self.cursor.position.1 = line_no + 1;
    //         true
    //     }
    // }

    pub fn insert_empty_line_at(&mut self, _focused: bool, _n: u32) {}

    pub fn insert_empty_line(&mut self, focused: bool) {
        self.lines.push(Line::empty(focused));
    }

    // fn get_focused_line(&self) -> Option<&Line> {
    //     self.lines.iter().find(|&line| line.has_focus())
    // }

    // TODO
    // fn update_lines(&mut self, anchor: u32) {
    //     self.lines
    //         .iter()
    //         .filter(|&line| line.line_number > anchor)
    //         .col;
    // }
    pub fn insert_line_with_text(&mut self, text: String) {
        self.lines.push(Line::new(text, false));
    }

    // fishy naming here
    // it should insert a Line at a given line_number
    pub fn insert_line_at_pos(&mut self, line: Line, line_number: u32) {
        if self.lines.len() < (line_number as usize) {
            panic!("too far into the unknown");
        } else {
            self.lines.insert(line_number as usize, line);
        }
    }

    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn nth_line(&self, n: u32) -> Option<&Line> {
        self.lines.get(n as usize)
    }
}

// struct TypeHere {

// }
