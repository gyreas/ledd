// WARN: spaghetti code
// TODO: create a new branch, use a different implementation

#[derive(Debug)]
pub struct Cursor {
    pub caret: char,
    position: u32, // zero-indexed
}

impl Cursor {
    pub fn new(caret: char) -> Cursor {
        Cursor { caret, position: 0 }
    }
    fn get_pos(&self) -> u32 {
        self.position
    }
    fn print_pos(&self) {
        let mut cp = String::new(); // line showing  cursor position with caret
        let p = self.position;
        for _ in 0..p {
            cp.push_str(" ");
        }
        cp.push_str(stringify!(cs));
    }
}
// edit buffer that holds text
#[derive(Debug)]
pub struct Line {
    pub text: String,
    pub cursor: Option<Cursor>,
    pub line_number: u32,
}

impl Line {
    fn new(text: String, cursor: Option<Cursor>, line_number: u32) -> Line {
        Line {
            text,
            cursor,
            line_number,
        }
    }

    fn has_focus(&self) -> bool {
        self.cursor.is_some()
    }
    fn get_cursor_pos(&self) -> u32 {
        if let Some(cs) = &self.cursor {
            cs.get_pos()
        } else {
            panic!("No cursor!");
        }
    }

    fn print_line(&self) {
        if let Some(cs) = &self.cursor {
            cs.print_pos();
            println!("{}", self.text);
            cs.print_pos();
        } else {
            println!("{}", self.text);
        }
    }
}

// use Builder pattern
// try
#[derive(Debug)]
pub struct Buffer {
    filename: String,
    lines: Vec<Line>,
    line_count: usize,
}

impl Buffer {
    pub fn empty() -> Buffer {
        Buffer {
            filename: String::new(),
            lines: Vec::new(),
            line_count: 0,
        }
    }

    pub fn init(filename: String, text: String) -> Buffer {
        let mut lines = Vec::new();
        for (i, line) in text.split_terminator('\n').enumerate() {
            lines.push(Line {
                text: line.to_owned(),
                cursor: if i == 0 { Some(Cursor::new('=')) } else { None },
                line_number: (i as u32),
            })
        }

        let line_count = lines.len();
        Buffer {
            filename,
            lines,
            line_count,
        }
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    // pub fn goto_next_line(&self) -> {}
    // pub fn add_newline(&self) {
    //     self.insert_line();
    //     line
    // }

    fn get_focused_line(&self) -> Option<&Line> {
        self.lines.iter().find(|&line| line.has_focus())
    }

    // TODO
    fn update_lines(&mut self, anchor: u32) {
        self.lines
            .iter()
            .filter(|&line| line.line_number > anchor)
            .col;
    }
    pub fn insert_line_with_text(&mut self, text: String) -> u32 {
        let curr_pos = self.get_focused_line().unwrap().line_number;
        self.lines.push(Line::new(text, None, curr_pos + 1));
        curr_pos
    }

    // fishy naming here
    pub fn insert_line_at_pos(&mut self, line: String, line_number: u32) {
        if self.lines.len() < (line_number as usize) {
            panic!("too far into the unknown");
        } else {
            if self
                .lines
                .iter()
                .find(|l| l.line_number == line_number)
                .is_some()
            {
                let line = Line {
                    text: line,
                    cursor: None,
                    line_number,
                };
                self.lines.push(line);
            }
        }
    }

    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn nth_line(&self, n: u32) -> Option<&Line> {
        self.lines.iter().find(|&line| line.line_number == n)
    }
}
