use std::io::{BufReader, Read};

use crate::{error::Result, token::Token};

pub struct Lexer {
    reader: BufReader<Box<dyn Read>>,
    line_number: usize,
    char_number: usize,
}

impl Lexer {
    pub fn new(input: Box<dyn Read>) -> Self {
        Lexer {
            reader: BufReader::new(input),
            line_number: 1,
            char_number: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let mut buf = [0];
        let bytes_read = self.reader.read(&mut buf).unwrap();
        let c = buf[0] as char;

        self.char_number += 1;
        if c == '\n' {
            self.line_number += 1;
            self.char_number = 0;
        }

        (bytes_read > 0).then_some(c)
    }
}

impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.next_char()?;
        print!("{}", c);
        unimplemented!()
    }
}
