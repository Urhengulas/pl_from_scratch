#[derive(Clone, Debug)]
pub struct InputStream {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl InputStream {
    pub fn new(s: &str) -> Self {
        Self {
            input: s.chars().map(|c| c).collect(),
            pos: 0,
            line: 0,
            col: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        Some(*self.input.get(self.pos)?)
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        if matches!(c, '\n') {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    pub fn eof(&self) -> bool {
        matches!(self.peek(), None)
    }

    pub fn croak(&self, err_msg: &str) {
        panic!("{}:{} – {}", self.line, self.col, err_msg);
    }
}

fn main() {
    let mut input = InputStream::new("let a = 1 + 2\nprint(a)");
    dbg!(&input);

    while input.eof() == false {
        let InputStream {
            pos,
            line,
            col,
            input: _,
        } = input;
        println!(
            "{}\t(pos: {}, line: {}, col: {})",
            input.next().unwrap(),
            pos,
            line,
            col
        );
    }
}
