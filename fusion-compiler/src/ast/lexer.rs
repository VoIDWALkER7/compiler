pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

pub struct TextSpan{
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan{
    pub fn new(start:usize, end: usize, literal:String) -> Self {
        TextSpan{ start, end, literal}
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a>{
    input: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {

    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable()}
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.peek().map(|c : &char |)
    }

    fn is_number_start(c: &char) -> bool {
        match &self.peek() {
            Some(c: &&char ) => c.is_digit(radix:10),
            None => false
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
}