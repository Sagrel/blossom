use tracing::{event, span};

use crate::token::{self, Token};

pub type AstIdx = usize;
pub type TokenIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Number {
        value: token::Symbol,
    },
    Identifier {
        name: token::Symbol,
    },
    BinaryOp {
        lhs: AstIdx,
        rhs: AstIdx,
        op: token::Kind,
    },
    If {
        cond: AstIdx,
        if_branch: AstIdx,
        else_branch: Option<AstIdx>,
    },
    Loop {
        body: AstIdx,
    },
    UnaryOp {
        expr: AstIdx,
        op: token::Kind,
    },
    Call {
        callee: AstIdx,
        args: Vec<AstIdx>,
    },
    Function {
        params: Vec<AstIdx>,
        result: AstIdx,
        body: AstIdx,
    },
    Block {
        statements: Vec<AstIdx>,
    },
    Import,
    Return {
        expr: AstIdx,
    },
    Error,
}

pub struct Node {
    pub kind: Kind,
    pub span: (TokenIdx, TokenIdx),
}

pub struct Module {
    pub definitions: Vec<AstIdx>,
    ast: Vec<Node>,
}

impl Module {
    pub fn get(&self, idx: AstIdx) -> &Node {
        &self.ast[idx]
    }
}

struct Parser {
    tokens: Vec<token::Token>,
    pos: TokenIdx,
    nodes: Vec<Node>,
}

impl Parser {
    fn new(tokens: Vec<token::Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            nodes: Vec::new(),
        }
    }

    fn peak(&self) -> Option<&token::Token> {
        self.tokens.get(self.pos)
    }

    fn save_ast(&mut self, ast: Node) -> AstIdx {
        event!(tracing::Level::DEBUG, "Saving AST node: {:?}", ast.kind);
        self.nodes.push(ast);
        self.nodes.len() - 1
    }
    fn consume(&mut self) -> token::Token {
        let token = self.tokens[self.pos];
        event!(tracing::Level::DEBUG, "Consuming token {:?}", token.kind);
        self.pos += 1;
        token
    }

    fn get(&self, idx: AstIdx) -> &Node {
        &self.nodes[idx]
    }

    fn parse_atom(&mut self) -> AstIdx {
        let token = self.consume();
        let span = span!(tracing::Level::DEBUG, "parse_atom", token = ?token.kind);
        let _enter = span.enter();
        match token.kind {
            token::Kind::Number(symbol) => self.save_ast(Node {
                kind: Kind::Number { value: symbol },
                span: (self.pos, self.pos + 1),
            }),
            token::Kind::Identifier(symbol) => self.save_ast(Node {
                kind: Kind::Identifier { name: symbol },
                span: (self.pos, self.pos + 1),
            }),
            token::Kind::If => {
                let cond = self.parse_expresion(0);
                let if_branch = self.parse_expresion(0);
                let else_branch = if self.peak().map_or(false, |t| t.kind == token::Kind::Else) {
                    self.consume(); // consume 'else'
                    Some(self.parse_expresion(0))
                } else {
                    None
                };
                self.save_ast(Node {
                    kind: Kind::If {
                        cond,
                        if_branch,
                        else_branch,
                    },
                    span: (self.pos, self.pos + 1),
                })
            }
            token::Kind::LBrace => {
                let mut statements = Vec::new();
                while self.peak().map_or(false, |t| t.kind != token::Kind::RBrace) {
                    statements.push(self.parse_expresion(0));
                }
                self.consume(); // consume 'RBrace'
                self.save_ast(Node {
                    kind: Kind::Block { statements },
                    span: (self.pos, self.pos + 1),
                })
            }
            token::Kind::Return => {
                let expr = self.parse_expresion(0);
                self.save_ast(Node {
                    kind: Kind::Return { expr },
                    span: (self.pos, self.pos + 1),
                })
            }
            _ => {
                // Handle unexpected token
                self.save_ast(Node {
                    kind: Kind::Error,
                    span: (self.pos, self.pos + 1),
                })
            }
        }
    }

    fn parse_binary_op(&mut self, lhs: AstIdx, precedence: usize) -> AstIdx {
        let span = span!(tracing::Level::DEBUG, "parse_binary_op", lhs = lhs, precedence = precedence);
        let _enter = span.enter();
        let mut lhs = lhs;
        while let Some(token) = self.peak() {
            let op_precedence = match token.kind {
                token::Kind::ColonEqual | token::Kind::Equal => 1,
                token::Kind::Plus | token::Kind::Minus => 10,
                token::Kind::Multiply | token::Kind::Divide => 20,
                token::Kind::LessThan
                | token::Kind::GreaterThan
                | token::Kind::LessThanOrEqual
                | token::Kind::GreaterThanOrEqual => 30,
                token::Kind::EqualEqual | token::Kind::NotEqual => 40,
                token::Kind::Dot | token::Kind::Arrow => 50,
                token::Kind::ColonColon => 60,
                _ => break,
            };

            if op_precedence < precedence {
                break;
            }

            let op = self.consume().kind; // consume operator
            let rhs = self.parse_atom();
            let ast = Node {
                kind: Kind::BinaryOp { lhs, rhs, op },
                span: (self.get(lhs).span.0, self.get(rhs).span.1),
            };
            lhs = self.save_ast(ast);
        }
        lhs
    }

    fn parse_expresion(&mut self, precedence: usize) -> AstIdx {
        let span = span!(tracing::Level::DEBUG, "parse_expresion", precedence = precedence);
        let _enter = span.enter();
        let lhs = self.parse_atom();
        self.parse_binary_op(lhs, precedence)
    }

    fn parse_program(mut self) -> Module {
        let mut definitions = Vec::new();
        while self.peak().is_some() {
            definitions.push(self.parse_expresion(0));
        }
        Module {
            definitions,
            ast: self.nodes,
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Module {
    Parser::new(tokens).parse_program()
}
