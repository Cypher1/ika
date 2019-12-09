use super::tokens::*;
use super::tree::Tree;
use super::ast::Visitor;

// Walks the AST interpreting it.
pub struct Interpreter;

impl Default for Interpreter {
    fn default () -> Interpreter {
        Interpreter{}
    }
}

impl Visitor<Tree<Token>, i32, i32> for Interpreter {
    fn visit_root(&mut self, expr: &Tree<Token>) -> i32 {
        self.visit(expr)
    }
    fn visit(&mut self, expr: &Tree<Token>) -> i32 {
        match expr.value.tok_type {
            TokenType::Error => {
                return -1; // "#err illegal err".to_string();
            }
            TokenType::Whitespace => {
                return -2; // "#err illegal whitespace".to_string();
            }
            TokenType::Unknown => {
                return -3; // "#err illegal unknown".to_string();
            }
            TokenType::Sym => {
                return -4; // "#err unknown symbol".to_string();
            }
            TokenType::Bracket => {
                // TODO: Require a single child.
                return self.visit(&expr.children[1]);
            }
            TokenType::Op => {
                // TODO: require 2 children
                match expr.value.value.as_str() {
                    "*" => return expr.children.iter().fold(1, |acc, x| acc * self.visit(x)),
                    "+" => return expr.children.iter().fold(0, |acc, x| acc + self.visit(x)),
                    "/" => {
                        // TODO: require divisibility
                        return self.visit(&expr.children[0]) / self.visit(&expr.children[1]);
                    }
                    "-" => {
                        return self.visit(&expr.children[0]) - self.visit(&expr.children[1]);
                    }
                    "^" => {
                        // TODO: require pos pow
                        return i32::pow(
                            self.visit(&expr.children[0]),
                            self.visit(&expr.children[1]) as u32,
                        );
                    }
                    _x => {
                        return -6; // x.to_string()+"?"
                    }
                }
            }
            TokenType::NumLit => {
                return expr.value.value.parse().unwrap();
            }
        }
    }
}
