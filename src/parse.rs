use crate::{ast::Node, lex::Lexer};

pub struct Parser {
    tokens: Lexer,
}

impl Parser {
    pub fn new(tokens: Lexer) -> Parser {
        Parser { tokens }
    }
}

impl Iterator for Parser {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
