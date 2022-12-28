pub mod error;
pub mod tokens;

use std::{
    intrinsics::unreachable,
    iter::Peekable,
    str::Chars,
    string::ParseError,
};

use tokens::Token;

use self::error::LexerError;
use crate::compiler::lexer::tokens::{
    SqlKeyword,
    Token::Keyword,
};

pub struct Lexer<'a> {
    input: String,
    buffer: Peekable<Chars<'a>>,
    start_of_token: usize,
    cursor: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.to_owned(),
            buffer: input.chars().peekable(),
            start_of_token: 0,
            cursor: 0,
            line: 1,
        }
    }

    pub fn prepare(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<LexerError> = Vec::new();

        while !self.is_at_end() {
            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        use Token::*;

        todo!()
    }

    fn next(&mut self) -> Option<char> {
        match self.buffer.next() {
            None => None,
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.cursor = 0;
                    self.start_of_token = 0;
                } else {
                    self.cursor += 1;
                }
                Some(s)
            },
        }
    }

    fn is_at_end(&mut self) -> bool {
        if self.buffer.peek() == None {
            true
        }
        false
    }

    fn make_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(char) = self.next() {
            if !char.is_whitespace() {
                word.push(char);
            } else {
                break;
            }
        }
        word
    }

    fn make_keyword(&mut self) -> Result<SqlKeyword, LexerError> {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                if !Self::is_keyword_start(ch) {
                    self.make_word();
                    return self.report_err(None);
                }
            }
        }

        todo!()
    }

    fn is_keyword_start(char: char) -> bool {
        match char {
            'A' | 'a' => true,
            'B' | 'b' => true,
            'C' | 'c' => true,
            'D' | 'd' => true,
            'E' | 'e' => true,
            'F' | 'f' => true,
            'I' | 'i' => true,
            'J' | 'j' => true,
            'L' | 'l' => true,
            'N' | 'n' => true,
            'O' | 'o' => true,
            'R' | 'r' => true,
            'S' | 's' => true,
            'T' | 't' => true,
            'U' | 'u' => true,
            'W' | 'w' => true,
            _ => false,
        }
    }

    fn report_err(&self, expected: Option<Token>) -> Err(LexerError) {
        Err(LexerError {
            input: self.input.clone(),
            line: self.line,
            start_of_token: self.start_of_token,
            cursor: self.cursor,
            expected,
        })
    }
}
