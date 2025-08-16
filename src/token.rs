use string_interner::{
    StringInterner,
    backend::{Backend, StringBackend},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Number(<StringBackend as Backend>::Symbol),
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
    Import,
    // Fallbacks
    Identifier(<StringBackend as Backend>::Symbol),
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub span: (usize, usize),
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    interner: StringInterner<StringBackend>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input,
            pos: 0,
            interner: StringInterner::default(),
        }
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> (usize, &'a str) {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.pos += 1;
            } else {
                break;
            }
        }
        (start, &self.input[start..self.pos])
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next(&mut self) -> Option<Token> {
        self.consume_while(char::is_whitespace);

        // Parse operators
        let (start, cosumed) = self.consume_while(|c| "+-*/<>=,.".contains(c));
        if !cosumed.is_empty() {
            let kind = match cosumed {
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
                _ => Kind::Unknown,
            };
            return Some(Token {
                kind,
                span: (start, self.pos),
            });
        }

        // Parse delimiters
        match self.peek() {
            Some('(') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::LParen,
                    span: (self.pos - 1, self.pos),
                });
            }
            Some(')') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::RParen,
                    span: (self.pos - 1, self.pos),
                });
            }
            Some('{') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::LBrace,
                    span: (self.pos - 1, self.pos),
                });
            }
            Some('}') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::RBrace,
                    span: (self.pos - 1, self.pos),
                });
            }
            Some('[') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::LBracket,
                    span: (self.pos - 1, self.pos),
                });
            }
            Some(']') => {
                self.pos += 1;
                return Some(Token {
                    kind: Kind::RBracket,
                    span: (self.pos - 1, self.pos),
                });
            }
            _ => {}
        }

        // Parse numbers
        let (start, consumed) = self.consume_while(char::is_numeric);
        if !consumed.is_empty() {
            return Some(Token {
                kind: Kind::Number(self.interner.get_or_intern(consumed)),
                span: (start, self.pos),
            });
        }

        // Parse identifiers
        let (start, consumed) = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        if !consumed.is_empty() {
            match consumed {
                "if" => {
                    return Some(Token {
                        kind: Kind::If,
                        span: (start, self.pos),
                    });
                }
                "else" => {
                    return Some(Token {
                        kind: Kind::Else,
                        span: (start, self.pos),
                    });
                }
                "for" => {
                    return Some(Token {
                        kind: Kind::For,
                        span: (start, self.pos),
                    });
                }
                "in" => {
                    return Some(Token {
                        kind: Kind::In,
                        span: (start, self.pos),
                    });
                }
                "import" => {
                    return Some(Token {
                        kind: Kind::Import,
                        span: (start, self.pos),
                    });
                }
                "and" => {
                    return Some(Token {
                        kind: Kind::And,
                        span: (start, self.pos),
                    });
                }
                "or" => {
                    return Some(Token {
                        kind: Kind::Or,
                        span: (start, self.pos),
                    });
                }
                "not" => {
                    return Some(Token {
                        kind: Kind::Not,
                        span: (start, self.pos),
                    });
                }
                _ => {
                    return Some(Token {
                        kind: Kind::Identifier(self.interner.get_or_intern(consumed)),
                        span: (start, self.pos),
                    });
                }
            }
        }

        if self.peek().is_some() {
            // If we reach here, we have an unknown character
            self.pos += 1; // Consume the unknown character
            return Some(Token {
                kind: Kind::Unknown,
                span: (self.pos - 1, self.pos),
            });
        }

        // If we reach here, we have no valid token
        return None;
    }
    // Additional methods for parsing would go here
}

pub fn parse(input: &str) -> (Vec<Token>, StringInterner<StringBackend>) {
    let mut parser = Parser::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = parser.next() {
        tokens.push(token);
    }

    (tokens, parser.interner)
}
