#[derive(Clone, Debug)]
struct InputStream {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl InputStream {
    pub fn new(s: &str) -> Self {
        todo!()
    }

    pub fn peek(&self) -> Option<char> {
        todo!()
    }

    pub fn next(&mut self) -> Option<char> {
        todo!()
    }

    pub fn eof(&self) -> bool {
        todo!()
    }

    pub fn croak(&self, err_msg: &str) {
        todo!()
    }
}

fn main() {
    let input = InputStream::new("let a = 1 + 2");
    dbg!(input);
}
