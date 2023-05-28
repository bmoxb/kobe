use std::io::{Read, Seek};

use crate::{ast::Node, lex::Lexer};

pub struct Parser<R> {
    tokens: Lexer<R>,
}

impl<R> Parser<R> {
    pub fn new(tokens: Lexer<R>) -> Self {
        Parser { tokens }
    }
}

impl<R: Read + Seek> Iterator for Parser<R> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        for token in &mut self.tokens {
            println!("{:?}", token);
        }
        unimplemented!()
    }
}
