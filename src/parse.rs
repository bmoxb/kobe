use std::io::Read;

use crate::{ast, lex::Lexer};

pub struct Parser<R> {
    tokens: Lexer<R>,
}

impl<R> Parser<R> {
    pub fn new(tokens: Lexer<R>) -> Self {
        Parser { tokens }
    }
}

impl<R: Read> Iterator for Parser<R> {
    type Item = ast::Stat;

    fn next(&mut self) -> Option<Self::Item> {
        for token in &mut self.tokens {
            match token {
                Ok(token) => println!("{}", token),
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }
        }
        unimplemented!()
    }
}
