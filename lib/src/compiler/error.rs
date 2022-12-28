use super::lexer::error::LexerError;
use super::lexer::tokens::Token;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexical error")]
    Lexer(#[source] LexerError),
    #[error("{0}")]
    Parser(String),
    #[error("{0}")]
    Preprocessor(String),
    #[error("{0}")]
    Optimizer(String),
    #[error("{0}")]
    Codegen(String),
}
