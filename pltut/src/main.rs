#[derive(Clone, Debug)]
struct InputStream {
    input: Vec<char>,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
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

trait Checker {
    fn next_is_comment(&self) -> bool;
    fn next_is_id(&self) -> bool;
    fn next_is_num(&self) -> bool;
    fn next_is_op(&self) -> bool;
    fn next_is_punc(&self) -> bool;
    fn next_is_str_bound(&self) -> bool;
}

impl Checker for char {
    fn next_is_comment(&self) -> bool {
        matches!(self, '#')
    }
    fn next_is_id(&self) -> bool {
        self.is_ascii_alphabetic()
    }
    fn next_is_num(&self) -> bool {
        self.is_ascii_digit()
    }
    fn next_is_op(&self) -> bool {
        ['+', '-', '*', '/', '%', '=', '&', '|', '<', '>', '!'].contains(self)
    }
    fn next_is_punc(&self) -> bool {
        [',', ';', '(', ')', '{', '}', '[', ']'].contains(self)
    }
    fn next_is_str_bound(&self) -> bool {
        matches!(self, '"')
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    Punc(char),
    Num(i32),
    Str(String),
    Kw(String),
    Var(String),
    Op(String),
}

#[derive(Clone, Debug)]
pub struct TokenStream(InputStream);

impl TokenStream {
    pub fn new(s: &str) -> Self {
        Self(InputStream::new(s))
    }

    fn next(&mut self) -> Token {
        self.skip_whitespace();

        match self.0.peek() {
            Some(c) => {
                if c.next_is_comment() {
                    self.skip_comment();
                    self.next()
                } else if c.next_is_str_bound() {
                    self.read_str()
                } else if c.next_is_num() {
                    self.read_num()
                } else if c.next_is_punc() {
                    self.read_punc()
                } else if c.next_is_op() {
                    self.read_op()
                } else if c.next_is_id() {
                    self.read_id()
                } else {
                    panic!("Can't handle char: '{}'", c)
                }
            }
            None => panic!("eof"),
        }
    }

    fn read_id(&mut self) -> Token {
        let id_str: String = self.read_while(|c| c.next_is_id()).into_iter().collect();
        if ["if", "then", "else", "lambda", "λ", "true", "false"].contains(&&*id_str) {
            Token::Kw(id_str)
        } else {
            Token::Var(id_str)
        }
    }

    fn read_num(&mut self) -> Token {
        let num_str: String = self.read_while(|c| c.next_is_num()).into_iter().collect();
        Token::Num(num_str.parse().unwrap())
    }

    fn read_op(&mut self) -> Token {
        let op_str = self.read_while(|c| c.next_is_op()).into_iter().collect();
        Token::Op(op_str)
    }

    fn read_punc(&mut self) -> Token {
        let punc_char = self.0.next().unwrap();
        Token::Punc(punc_char)
    }

    fn read_str(&mut self) -> Token {
        assert_eq!(self.0.next(), Some('"'));
        let str_str: String = self
            .read_while(|c| !c.next_is_str_bound())
            .into_iter()
            .collect();
        assert_eq!(self.0.next(), Some('"'));

        Token::Str(format!("\"{}\"", str_str))
    }

    fn read_while(&mut self, accept: impl Fn(char) -> bool) -> Vec<char> {
        let mut buf = Vec::new();
        while accept(self.0.peek().unwrap()) {
            buf.push(self.0.next().unwrap());
        }
        buf
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.0.peek(), Some(' ') | Some('\t') | Some('\n')) {
            self.0.next();
        }
    }

    fn skip_comment(&mut self) {
        while self.0.col > 0 {
            self.0.next();
        }
    }
}

fn main() {
    let mut token = TokenStream::new("let a = 1 + 2");
    dbg!(&token);

    while token.0.eof() == false {
        dbg!(token.next());
    }
}
