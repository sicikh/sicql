use std::fmt::{
    write,
    Display,
    Formatter,
};

use super::Token;

#[derive(thiserror::Error, Debug)]
pub struct LexerError {
    pub input: String,
    pub line: usize,
    pub start_of_token: usize,
    pub cursor: usize,
    pub expected: Option<Token>,
}

/* TODO: pretty error output
// FIXME: Some devil shyt. Maybe use methods of formatter?
impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_line = self.input.lines().nth(self.line - 1).unwrap_or_default();
        let indentation_width = self.line.to_string().len() + 1;

        write!(
            f,
            "Error[lex]: unknown token.\n{}|\n{} | {}\n{}| {}{} help: \
        consider using '{}'",
            " ".repeat(indentation_width),
            self.line,
            error_line,
            " ".repeat(indentation_width),
            " ".repeat(self.start_of_token),
            "^".repeat(self.cursor - self.start_of_token + 1),
            self.expected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repl_format() {
        let error = LexerError {
            input: "SLECT * FROM test1;\n".to_string(),
            line: 1,
            start_of_token: 0,
            cursor: 4,
            expected: Token::Select,
        };

        assert_eq!(
            "Error[lex]: unknown token.
  |
1 | SLECT * FROM test1;
  | ^^^^^ help: consider using 'SELECT'",
            format!("{error}")
        );
    }

    #[test]
    fn multiline_format() {
        let error = LexerError {
            input: "SELECT * FROM test1;\nSELECT * FRM test2;\n".to_string(),
            line: 2,
            start_of_token: 9,
            cursor: 11,
            expected: Token::From,
        };

        assert_eq!(
            "Error[lex]: unknown token.
  |
2 | SELECT * FRM test2;
  |          ^^^ help: consider using 'FROM'",
            format!("{error}")
        );
    }

    #[test]
    fn indentation() {
        let error = LexerError {
            input: "s\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\ns\nSEL * FROM test1;\n"
                .to_string(),
            line: 20,
            start_of_token: 0,
            cursor: 2,
            expected: Token::Select,
        };

        assert_eq!(
            "Error[lex]: unknown token.
   |
20 | SEL * FROM test1;
   | ^^^ help: consider using 'SELECT'",
            format!("{error}")
        );
    }
} */
