use std::io::{BufReader, Read};

use crate::{
    error::Result,
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
        self.next_char_if(lexeme, |c| c == target)
    }

    fn next_char_if(&mut self, lexeme: &mut String, f: impl Fn(char) -> bool) -> bool {
        if let Some(c) = self.next_char_no_update() {
            if f(c) {
                self.update_position_tracking(c);
                lexeme.push(c);
                return true;
            } else {
                self.peeked_char = Some(c);
            }
        }
        false
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
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lexeme = String::new();

        let c = self.next_char(&mut lexeme)?;

        let tok_type = match c {
            '=' => {
                if self.next_char_if_equals(&mut lexeme, '=') {
                    TokenType::Equivalent
                } else {
                    TokenType::Assign
                }
            }
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '(' => TokenType::OpenBracket,
            ')' => TokenType::CloseBracket,
            '[' => TokenType::OpenSquare,
            ']' => TokenType::CloseSquare,
            '+' => TokenType::Plus,
            '-' => {
                if self.next_char_if_equals(&mut lexeme, '>') {
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => TokenType::Times,
            '/' => TokenType::Divide,
            '<' => {
                if self.next_char_if_equals(&mut lexeme, '=') {
                    TokenType::LessThanOrEqual
                } else {
                    TokenType::LessThan
                }
            }
            '>' => {
                if self.next_char_if_equals(&mut lexeme, '=') {
                    TokenType::GreaterThanOrEqual
                } else {
                    TokenType::GreaterThan
                }
            }
            '!' => {
                if self.next_char_if_equals(&mut lexeme, '=') {
                    TokenType::NotEquivalent
                } else {
                    TokenType::Not
                }
            }
            '0'..='9' => unimplemented!(),
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.next_char_if(&mut lexeme, is_ident_char) {}
                choose_identifier_or_keyword_token_type(&lexeme)
            }
            c if c.is_whitespace() => return self.next(),
            _ => unimplemented!(),
        };

        Some(Ok(Token {
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

fn choose_identifier_or_keyword_token_type(lexeme: &str) -> TokenType {
    match lexeme {
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    macro_rules! assert_lex {
        ($input:literal, $type:expr, $lexeme:literal, $line:literal, $char:literal) => {
            let expected = Token {
                tok_type: $type,
                lexeme: $lexeme.to_string(),
                line_number: $line,
                char_number: $char,
            };

            let cursor = Cursor::new($input);
            let mut l = Lexer::new(cursor);
            assert_eq!(l.next(), Some(Ok(expected)));
        };
    }

    #[test]
    fn simple_tokens() {
        assert_lex!("=", TokenType::Assign, "=", 1, 1);
        assert_lex!("==", TokenType::Equivalent, "==", 1, 2);
        assert_lex!(" :", TokenType::Colon, ":", 1, 2);
        assert_lex!(", ", TokenType::Comma, ",", 1, 1);
        assert_lex!("\n(", TokenType::OpenBracket, "(", 2, 1);
        assert_lex!(")\n", TokenType::CloseBracket, ")", 1, 1);
        assert_lex!(" [ ", TokenType::OpenSquare, "[", 1, 2);
        assert_lex!(" ] ", TokenType::CloseSquare, "]", 1, 2);
        assert_lex!("+", TokenType::Plus, "+", 1, 1);
        assert_lex!("-", TokenType::Minus, "-", 1, 1);
        assert_lex!("->", TokenType::Arrow, "->", 1, 2);
        assert_lex!("-\t>", TokenType::Minus, "-", 1, 1);
        assert_lex!("\t *", TokenType::Times, "*", 1, 3);
        assert_lex!("/ \t", TokenType::Divide, "/", 1, 1);
        assert_lex!("<", TokenType::LessThan, "<", 1, 1);
        assert_lex!(" <= ", TokenType::LessThanOrEqual, "<=", 1, 3);
        assert_lex!(">", TokenType::GreaterThan, ">", 1, 1);
        assert_lex!(" >= ", TokenType::GreaterThanOrEqual, ">=", 1, 3);
        assert_lex!("!", TokenType::Not, "!", 1, 1);
        assert_lex!("!=", TokenType::NotEquivalent, "!=", 1, 2);
    }

    #[test]
    fn identifiers_and_keywords() {
        assert_lex!("a", TokenType::Identifier, "a", 1, 1);
        assert_lex!("_", TokenType::Identifier, "_", 1, 1);
        assert_lex!(" ABC_123 ", TokenType::Identifier, "ABC_123", 1, 8);
        assert_lex!("\nif", TokenType::IfKeyword, "if", 2, 2);
    }

    #[test]
    fn number_literals() {
        // TODO
    }

    #[test]
    fn char_literals() {
        // TODO
    }

    #[test]
    fn string_literals() {
        // TODO
    }
}
