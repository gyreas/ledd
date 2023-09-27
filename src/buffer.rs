// edit buffer that holds text
#[derive(Debug)]
pub struct Line {
    pub text: String,
    pub cursor: Cursor,
    pub line_number: u32,
}

impl Line {
    fn new() -> Line {
        todo!();
    }

    fn get_cursor_pos(&self) -> u32 {
        todo!();
    }

    fn print_line(&self) -> String {
        self.text
    }
}

// use Builder pattern
// try
pub struct Buffer {
    filename: &str,
    lines: Vec<Line>,
    line_count: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        todo!();
    }

    pub fn set_filename(&self, filename: &str) -> Buffer {
        Buffer {
            filename,
            lines: self.lines,
            line_count: self.line_count,
        }
    }

    pub fn insert_line(&self, line: String) -> Buffer {
        Buffer {
            self.filename,
            self.lines

        }
    }
    pub fn get_filename(&self) -> &str {
        self.filename
    }

    pub fn nth_line(&self, n: usize) -> Line {
        todo!();
    }
}

