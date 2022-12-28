use self::error::CompilerError;

mod codegen;
mod error;
mod lexer;
mod parser;
mod planner;
mod preprocessor;

pub struct Compiler {
    query: String,
}

impl Compiler {
    pub fn new(query: &str) -> Self {
        Compiler {
            query: query.to_owned(),
        }
    }

    pub fn compile(&mut self) {
        todo!()
    }
}
