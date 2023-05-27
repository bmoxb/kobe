use std::io::{BufReader, Read};

use crate::{error::Result, token::Token};

pub struct Lexer {
    reader: BufReader<Box<dyn Read>>,
}

impl Lexer {
    pub fn new(input: Box<dyn Read>) -> Self {
        Lexer {
            reader: BufReader::new(input),
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
