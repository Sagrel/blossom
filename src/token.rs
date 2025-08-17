use string_interner::{
    StringInterner,
    backend::{Backend, StringBackend},
};
use tracing::{event, span};

pub type Symbol = <StringBackend as Backend>::Symbol;
pub type Interner = StringInterner<StringBackend>;



#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Number(Symbol),
    Text(Symbol),
    Plus,
    Minus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    EqualEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,
    ColonEqual,
    ColonColon,
    Arrow,
    And,
    Or,
    Not,
    Comma,
    Dot,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // Keywords
    If,
    Else,
    For,
    In,
    Return,
    Break,
    Continue,
    Loop,
    Import,
    External,
    // Fallbacks
    Identifier(Symbol),
    Unknown(Symbol),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub span: (usize, usize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    InNumber(usize),
    InDecimal(usize),
    InIdentifier(usize),
    InOperator(usize),
    InText(usize),
    InDelimiter(usize),
    InComment(usize),
    InUnknown(usize),
    Eof,    
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    interner: Interner,
    state: ParserState,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input,
            pos: 0,
            interner: StringInterner::default(),
            state: ParserState::Start,
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn create_token(&mut self) {
        span!(tracing::Level::DEBUG, "create_token", state = ?self.state);
        let token = match self.state {            
            ParserState::InIdentifier(start) => {
                match &self.input[start..self.pos] {
                    "if" => Token { kind: Kind::If, span: (start, self.pos) },
                    "else" => Token { kind: Kind::Else, span: (start, self.pos) },
                    "for" => Token { kind: Kind::For, span: (start, self.pos) },
                    "in" => Token { kind: Kind::In, span: (start, self.pos) },
                    "import" => Token { kind: Kind::Import, span: (start, self.pos) },
                    "and" => Token { kind: Kind::And, span: (start, self.pos) },
                    "or" => Token { kind: Kind::Or, span: (start, self.pos) },
                    "not" => Token { kind: Kind::Not, span: (start, self.pos) },
                    "return" => Token { kind: Kind::Return, span: (start, self.pos) },
                    "break" => Token { kind: Kind::Break, span: (start, self.pos) },
                    "continue" => Token { kind: Kind::Continue, span: (start, self.pos) },
                    "loop" => Token { kind: Kind::Loop, span: (start, self.pos) },
                    "external" => Token { kind: Kind::External, span: (start, self.pos) },
                    consumed => Token { kind: Kind::Identifier(self.interner.get_or_intern(consumed)), span: (start, self.pos) },
                }
            }
            ParserState::InOperator(start) => {
                let operator = &self.input[start..self.pos];
                let kind = match operator {
                    "+" => Kind::Plus,
                    "-" => Kind::Minus,
                    "*" => Kind::Multiply,
                    "/" => Kind::Divide,
                    "<" => Kind::LessThan,
                    ">" => Kind::GreaterThan,
                    "=" => Kind::Equal,
                    "<=" => Kind::LessThanOrEqual,
                    ">=" => Kind::GreaterThanOrEqual,
                    "==" => Kind::EqualEqual,
                    "!=" => Kind::NotEqual,
                    ":=" => Kind::ColonEqual,
                    "::" => Kind::ColonColon,
                    "->" => Kind::Arrow,
                    "," => Kind::Comma,
                    "." => Kind::Dot,
                    _ => Kind::Unknown(self.interner.get_or_intern(operator)),
                };
                Token { kind, span: (start, self.pos) }
            }
            ParserState::InDelimiter(start) => {
                let delimiter = &self.input[start..self.pos];
                let kind = match delimiter {
                    "(" => Kind::LParen,
                    ")" => Kind::RParen,
                    "{" => Kind::LBrace,
                    "}" => Kind::RBrace,
                    "[" => Kind::LBracket,
                    "]" => Kind::RBracket,
                    _ => Kind::Unknown(self.interner.get_or_intern(delimiter)),
                };
                Token { kind, span: (start, self.pos) }
            }
            ParserState::InNumber(start) | ParserState::InDecimal(start) => {
                Token {
                    kind: Kind::Number(self.interner.get_or_intern(&self.input[start..self.pos])),
                    span: (start, self.pos),
                }
            }
            ParserState::InText(start) => {
                Token {
                    kind: Kind::Text(self.interner.get_or_intern(&self.input[start..self.pos])),
                    span: (start, self.pos),
                }
            }
            ParserState::InUnknown(start) => {
                Token {
                    kind: Kind::Unknown(self.interner.get_or_intern(&self.input[start..self.pos])),
                    span: (start, self.pos),
                }
            }
            ParserState::InComment(_) | ParserState::Eof | ParserState::Start => {
                unreachable!("Unexpected parser state when creating token: {:?}", self.state)
            }            
        };
        event!(tracing::Level::DEBUG, "Creating token: {:?}", token);
        self.tokens.push(token);
        self.state = ParserState::Start; // Reset state after creating token
    }

    fn consume(&mut self) -> usize {
        event!(tracing::Level::DEBUG, "Consuming character {}",  self.input[self.pos..].chars().next().unwrap_or(' '));
        let start = self.pos;
        self.pos += 1;
        start
    }

    fn next(&mut self) {
        let Some(c) = self.peek() else {
            if self.state != ParserState::Eof || self.state != ParserState::Start {
                self.create_token(); // Create token for the last state
            }
            self.state = ParserState::Eof;
            return;
        };

        let span = span!(tracing::Level::DEBUG, "next", c = %c, state = ?self.state);
        let _enter = span.enter();

        match (self.state, c) {
            (ParserState::Start, _) if c.is_whitespace() => {
                self.consume(); // Skip whitespace
            }
            (ParserState::Start, _) if c.is_alphabetic() || c == '_' => {
                self.state = ParserState::InIdentifier(self.consume());
            }
            (ParserState::Start, _) if c.is_numeric() => {
                self.state = ParserState::InNumber(self.consume());
            }
            (ParserState::Start, _) if "+-*/<>:=,.".contains(c) => {
                self.state = ParserState::InOperator(self.consume());
            }
            (ParserState::Start, _) if "()[]{}".contains(c) => {
                self.state = ParserState::InDelimiter(self.consume());
            }
            (ParserState::Start, '"') => {
                self.state = ParserState::InText(self.consume());
            }
            (ParserState::Start, ';') => {
                self.state = ParserState::InComment(self.consume());
            }            
            (ParserState::Start, _) => {
                self.state = ParserState::InUnknown(self.consume());
            }    
            (ParserState::InUnknown(_), _) if c.is_alphanumeric() || "+-*/<>:=,._()[]{};\"".contains(c) => {
                self.create_token();
            }
            (ParserState::InIdentifier(_), _) if !c.is_alphanumeric() && c != '_' => {
                self.create_token();
            }
            (ParserState::InNumber(start), '.') => {
                self.consume(); // Consume the dot for decimal
                self.state = ParserState::InDecimal(start);
            }
            (ParserState::InNumber(_) | ParserState::InDecimal(_), _) if !c.is_numeric() && c != '_'  => {
                self.create_token();
            }
            (ParserState::InOperator(_), _) if !"+-*/<>:=,.".contains(c) => {
                self.create_token();
            }
            (ParserState::InDelimiter(_), _) => {
                self.create_token();
            }
            (ParserState::InText(_), '"') => {
                self.consume(); // Consume closing quote
                self.create_token();
            }
            (ParserState::InComment(_), '\n') => {
                self.consume(); // Consume newline to end comment
            }
            _ => {
                // Continue consuming characters in the current state
                self.consume();
            }
        }    
        //event!(tracing::Level::DEBUG, "State after processing character: {:?}", self.state);       
    }
    // Additional methods for parsing would go here
}


pub fn parse(input: &str) -> (Vec<Token>, Interner) {
    event!(tracing::Level::DEBUG, "Starting parsing input: {}", input);
    let mut parser = Parser::new(input);

    while parser.state != ParserState::Eof {
        parser.next();
    }

    (parser.tokens, parser.interner)
}
