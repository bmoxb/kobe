use std::io::{BufReader, Read};

use crate::{
    error::{Error, ErrorKind, LexicalErrorKind, Result},
    token::{Token, TokenType},
};

pub struct Lexer<R> {
    reader: BufReader<R>,
    line_number: usize,
    char_number: usize,
    peeked_char: Option<char>,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            reader: BufReader::new(input),
            line_number: 1,
            char_number: 0,
            peeked_char: None,
        }
    }

    // Read the next character from the buffer. Will return `None` if reached
    // the end of the file or input stream. This function will track the
    // position (line and character numbers) in the input. The read character
    // will be appened to the provide lexeme string also.
    fn next_char(&mut self, lexeme: &mut String) -> Option<char> {
        let c = self.next_char_no_update();
        if let Some(c) = c {
            self.update_position_tracking(c);
            lexeme.push(c);
        }
        c
    }

    // Peek the next character in the input and, if it is equal to the given
    // target character, do the following:
    // * Track position (line and character numbers) in the input.
    // * Append the character to the given lexeme.
    // * Return `true`.
    // If the peeked character is not equal then do not do the above and just
    // return `false`.
    fn next_char_if_equals(&mut self, lexeme: &mut String, target: char) -> bool {
        self.next_char_if(lexeme, |c| c == target).is_some()
    }

    fn next_char_if(&mut self, lexeme: &mut String, f: impl Fn(char) -> bool) -> Option<char> {
        if let Some(c) = self.next_char_no_update() {
            if f(c) {
                self.update_position_tracking(c);
                lexeme.push(c);
                return Some(c);
            } else {
                self.peeked_char = Some(c);
            }
        }
        None
    }

    fn next_char_no_update(&mut self) -> Option<char> {
        if self.peeked_char.is_some() {
            self.peeked_char.take()
        } else {
            let mut buf = [0];
            let bytes_read = self.reader.read(&mut buf).unwrap();
            let c = buf[0] as char;
            (bytes_read > 0).then_some(c)
        }
    }

    fn update_position_tracking(&mut self, c: char) {
        self.char_number += 1;
        if c == '\n' {
            self.line_number += 1;
            self.char_number = 0;
        }
    }

    fn handle_number_literal(&mut self, lexeme: &mut String) -> Result<TokenType> {
        let mut tt = TokenType::IntLiteral;

        while let Some(c) = self.next_char_if(lexeme, is_number_char) {
            if c == '.' {
                if tt == TokenType::IntLiteral {
                    tt = TokenType::FloatLiteral;
                } else {
                    return Err(self.new_error(LexicalErrorKind::InvalidFloatLiteral));
                }
            }
        }

        Ok(tt)
    }

    fn handle_ident_or_keyword(&mut self, lexeme: &mut String) -> TokenType {
        while self.next_char_if(lexeme, is_ident_char).is_some() {}

        match lexeme.as_str() {
            "do" => TokenType::DoKeyword,
            "end" => TokenType::EndKeyword,
            "for" => TokenType::ForKeyword,
            "while" => TokenType::WhileKeyword,
            "if" => TokenType::IfKeyword,
            "then" => TokenType::ThenKeyword,
            "else" => TokenType::ElseKeyword,
            "fn" => TokenType::FnKeyword,
            "return" => TokenType::ReturnKeyword,
            "and" => TokenType::AndKeyword,
            "or" => TokenType::OrKeyword,
            _ => TokenType::Identifier,
        }
    }

    fn handle_char_literal(&mut self, lexeme: &mut String) -> Result<TokenType> {
        if self.next_char_if_equals(lexeme, '\\') {
            if self.next_char_if(lexeme, is_escape_code_char).is_none() {
                return Err(self.new_error(LexicalErrorKind::InvalidEscapeCode));
            }
        } else if self.next_char_if(lexeme, is_valid_char_literal).is_none() {
            return Err(self.new_error(LexicalErrorKind::InvalidCharLiteral));
        }

        if self.next_char_if_equals(lexeme, '\'') {
            Ok(TokenType::CharLiteral)
        } else {
            Err(self.new_error(LexicalErrorKind::InvalidCharLiteral))
        }
    }

    fn new_error(&mut self, kind: LexicalErrorKind) -> Error {
        Error {
            kind: ErrorKind::Lexical(kind),
            line: String::new(), // TODO
            line_number: self.line_number,
            char_number: self.char_number,
            file_path: None, // TODO
        }
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lexeme = String::new();

        let c = self.next_char(&mut lexeme)?;

        let tok_type = match c {
            ':' => Ok(TokenType::Colon),
            ',' => Ok(TokenType::Comma),
            '(' => Ok(TokenType::OpenBracket),
            ')' => Ok(TokenType::CloseBracket),
            '[' => Ok(TokenType::OpenSquare),
            ']' => Ok(TokenType::CloseSquare),
            '+' => Ok(TokenType::Plus),
            '*' => Ok(TokenType::Times),
            '/' => Ok(TokenType::Divide),

            '=' => Ok(if self.next_char_if_equals(&mut lexeme, '=') {
                TokenType::Equivalent
            } else {
                TokenType::Assign
            }),

            '-' => Ok(if self.next_char_if_equals(&mut lexeme, '>') {
                TokenType::Arrow
            } else {
                TokenType::Minus
            }),

            '<' => Ok(if self.next_char_if_equals(&mut lexeme, '=') {
                TokenType::LessThanOrEqual
            } else {
                TokenType::LessThan
            }),

            '>' => Ok(if self.next_char_if_equals(&mut lexeme, '=') {
                TokenType::GreaterThanOrEqual
            } else {
                TokenType::GreaterThan
            }),

            '!' => Ok(if self.next_char_if_equals(&mut lexeme, '=') {
                TokenType::NotEquivalent
            } else {
                TokenType::Not
            }),

            '0'..='9' => self.handle_number_literal(&mut lexeme),

            'a'..='z' | 'A'..='Z' | '_' => Ok(self.handle_ident_or_keyword(&mut lexeme)),

            '\'' => self.handle_char_literal(&mut lexeme),

            _ if c.is_whitespace() => return self.next(),

            _ => Err(self.new_error(LexicalErrorKind::UnexpectedCharacter)),
        };

        Some(tok_type.map(|tok_type| Token {
            tok_type,
            lexeme,
            line_number: self.line_number,
            char_number: self.char_number,
        }))
    }
}

fn is_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn is_number_char(c: char) -> bool {
    matches!(c, '0'..='9' | '.')
}

fn is_escape_code_char(c: char) -> bool {
    matches!(c, '\\' | 'n' | 't' | '\'' | '0')
}

fn is_valid_char_literal(c: char) -> bool {
    !matches!(c, '\'' | '\n')
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    macro_rules! assert_token {
        ($input:literal, $type:expr, $lexeme:literal, $line_no:literal, $char_no:literal) => {
            let expected = Token {
                tok_type: $type,
                lexeme: $lexeme.to_string(),
                line_number: $line_no,
                char_number: $char_no,
            };

            let cursor = Cursor::new($input);
            let mut l = Lexer::new(cursor);
            assert_eq!(l.next(), Some(Ok(expected)));
        };
    }

    macro_rules! assert_error {
        ($input:literal, $kind:expr, $line:literal, $line_no:literal, $char_no:literal) => {
            let error = Error {
                kind: ErrorKind::Lexical($kind),
                line_number: $line_no,
                char_number: $char_no,
                line: $line.to_string(),
                file_path: None,
            };

            let cursor = Cursor::new($input);
            let mut l = Lexer::new(cursor);
            assert_eq!(l.next(), Some(Err(error)));
        };
    }

    #[test]
    fn simple_tokens() {
        assert_token!("=", TokenType::Assign, "=", 1, 1);
        assert_token!("==", TokenType::Equivalent, "==", 1, 2);
        assert_token!(" :", TokenType::Colon, ":", 1, 2);
        assert_token!(", ", TokenType::Comma, ",", 1, 1);
        assert_token!("\n(", TokenType::OpenBracket, "(", 2, 1);
        assert_token!(")\n", TokenType::CloseBracket, ")", 1, 1);
        assert_token!(" [ ", TokenType::OpenSquare, "[", 1, 2);
        assert_token!(" ] ", TokenType::CloseSquare, "]", 1, 2);
        assert_token!("+", TokenType::Plus, "+", 1, 1);
        assert_token!("-", TokenType::Minus, "-", 1, 1);
        assert_token!("->", TokenType::Arrow, "->", 1, 2);
        assert_token!("-\t>", TokenType::Minus, "-", 1, 1);
        assert_token!("\t *", TokenType::Times, "*", 1, 3);
        assert_token!("/ \t", TokenType::Divide, "/", 1, 1);
        assert_token!("<", TokenType::LessThan, "<", 1, 1);
        assert_token!(" <= ", TokenType::LessThanOrEqual, "<=", 1, 3);
        assert_token!(">", TokenType::GreaterThan, ">", 1, 1);
        assert_token!(" >= ", TokenType::GreaterThanOrEqual, ">=", 1, 3);
        assert_token!("!", TokenType::Not, "!", 1, 1);
        assert_token!("!=", TokenType::NotEquivalent, "!=", 1, 2);
    }

    #[test]
    fn identifiers_and_keywords() {
        assert_token!("a", TokenType::Identifier, "a", 1, 1);
        assert_token!("_", TokenType::Identifier, "_", 1, 1);
        assert_token!(" ABC_123 ", TokenType::Identifier, "ABC_123", 1, 8);
        assert_token!("\nif", TokenType::IfKeyword, "if", 2, 2);
    }

    #[test]
    fn number_literals() {
        assert_token!("0", TokenType::IntLiteral, "0", 1, 1);
        assert_token!("1234", TokenType::IntLiteral, "1234", 1, 4);
        assert_token!("1.\n", TokenType::FloatLiteral, "1.", 1, 2);
        assert_token!(" 123.456 ", TokenType::FloatLiteral, "123.456", 1, 8);
        assert_error!(".", LexicalErrorKind::UnexpectedCharacter, "", 1, 1);
        assert_error!("1.2.3", LexicalErrorKind::InvalidFloatLiteral, "", 1, 4);
    }

    #[test]
    fn char_literals() {
        assert_token!("'x'", TokenType::CharLiteral, "'x'", 1, 3);
        assert_token!(" ' ' ", TokenType::CharLiteral, "' '", 1, 4);
        assert_token!("'\\n'", TokenType::CharLiteral, "'\\n'", 1, 4);
        assert_token!("'\\t'", TokenType::CharLiteral, "'\\t'", 1, 4);
        assert_error!("'\\j'", LexicalErrorKind::InvalidEscapeCode, "", 1, 2);
        assert_error!("'", LexicalErrorKind::InvalidCharLiteral, "", 1, 1);
        assert_error!("''", LexicalErrorKind::InvalidCharLiteral, "", 1, 1);
        assert_error!("'xy'", LexicalErrorKind::InvalidCharLiteral, "", 1, 2);
        assert_error!("'\n'", LexicalErrorKind::InvalidCharLiteral, "", 1, 1);
    }

    #[test]
    fn string_literals() {
        // TODO
    }
}
