use std::fmt::Write;

use crate::{
    ast::{self, AstIdx, Module},
    token::{self, Interner},
};

struct Printer<'a> {
    module: &'a ast::Module,
    interner: &'a Interner,
    buffer: String,
}

impl<'a> Printer<'a> {
    fn indent(&mut self, indent: usize) {
        self.buffer.push_str(" ".repeat(indent).as_str());
    }

    fn print_op(&mut self, op: &token::Kind) -> std::fmt::Result {
        match op {
            token::Kind::Plus => write!(self.buffer, "+"),
            token::Kind::Minus => write!(self.buffer, "-"),
            token::Kind::Multiply => write!(self.buffer, "*"),
            token::Kind::Divide => write!(self.buffer, "/"),
            token::Kind::Equal => write!(self.buffer, "="),
            token::Kind::ColonEqual => write!(self.buffer, ":="),
            token::Kind::EqualEqual => write!(self.buffer, "=="),
            token::Kind::NotEqual => write!(self.buffer, "!="),
            token::Kind::LessThan => write!(self.buffer, "<"),
            token::Kind::GreaterThan => write!(self.buffer, ">"),
            token::Kind::LessThanOrEqual => write!(self.buffer, "<="),
            token::Kind::GreaterThanOrEqual => write!(self.buffer, ">="),
            _ => unreachable!("We should only have operators here!"), 
        }
    }

    fn print_node(&mut self, node: AstIdx, indent: usize) -> std::fmt::Result {
        match &self.module.get(node).kind {
            ast::Kind::BinaryOp { lhs, rhs, op } => {
                self.print_node(*lhs, indent)?;
                self.buffer.push(' ');
                self.print_op(op)?;
                self.buffer.push(' ');
                self.print_node(*rhs, indent + 2)?;
            }
            ast::Kind::UnaryOp { expr, op } => {
                self.print_op(op)?;
                self.print_node(*expr, indent)?;
            }
            ast::Kind::Call { callee, args } => {
                self.print_node(*callee, indent)?;
                self.buffer.push('(');
                for (i, arg) in args.iter().enumerate() {
                    self.print_node(*arg, indent)?;
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                }
                self.buffer.push(')');
            }
            ast::Kind::Function {
                params,
                result,
                body,
            } => {
                self.buffer.push_str("(");
                for (i, param) in params.iter().enumerate() {
                    self.print_node(*param, indent)?;
                    if i > 0 {
                        self.buffer.push_str(", ");
                    }
                }
                self.buffer.push_str(") -> ");
                self.print_node(*result, indent)?;
                self.buffer.push(' ');
                self.print_node(*body, indent)?;
            }
            ast::Kind::Block { statements } => {
                self.buffer.push_str("{\n");
                self.indent(indent + 2);
                for statement in statements {
                    self.print_node(*statement, indent + 2)?;
                    self.buffer.push('\n');
                }
                self.indent(indent);
                self.buffer.push('}');
            }
            ast::Kind::If {
                cond,
                if_branch,
                else_branch,
            } => {
                self.buffer.push_str("if ");
                self.print_node(*cond, indent)?;
                self.buffer.push(' ');
                self.print_node(*if_branch, indent)?;
                if let Some(else_branch) = else_branch {
                    self.buffer.push_str(" else ");
                    self.print_node(*else_branch, indent)?;
                }
            }
            ast::Kind::Loop { body } => {
                self.buffer.push_str("loop ");
                self.print_node(*body, indent)?;
            }
            ast::Kind::Import => {
                self.buffer.push_str("import");
            }
            ast::Kind::Error => {
                write!(self.buffer, "<Error>")?;
            }
            ast::Kind::Identifier { name } => {
                if let Some(name) = self.interner.resolve(*name) {
                    write!(self.buffer, "{}", name)?;
                } else {
                    self.buffer.push_str("<Unknown identifier>");
                }
            }
            ast::Kind::Number { value } => {
                if let Some(name) = self.interner.resolve(*value) {
                    write!(self.buffer, "{}", name)?;
                } else {
                    self.buffer.push_str("<Unknown number>");
                }
            }
            ast::Kind::Return { expr } => {
                self.buffer.push_str("return ");
                self.print_node(*expr, indent)?;
            }
        }
        Ok(())
    }
}

pub fn print(module: &Module, interner: &Interner) -> String {
    let mut printer = Printer {
        module,
        interner,
        buffer: String::new(),
    };
    for node in &module.definitions {
        printer.print_node(*node, 0).expect("Failed to print node");
        printer.buffer.push('\n');
    }
    printer.buffer
}
