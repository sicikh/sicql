use std::string::ParseError;

use crate::compiler::{
    lexer::tokens::Token,
    parser::{
        ast::AST,
        symbol_table::SymbolTable,
    },
};

mod ast;
mod symbol_table;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Result<(AST, SymbolTable), ParseError> {
        todo!()
    }
}
