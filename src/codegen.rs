use std::io::Read;

use crate::{error::Result, parse::Parser};

pub struct CodeGenerator<R> {
    nodes: Parser<R>,
}

impl<R: Read> CodeGenerator<R> {
    pub fn new(nodes: Parser<R>) -> Self {
        CodeGenerator { nodes }
    }

    pub fn generate_wasm(self) -> Result<Vec<u8>> {
        for _node in self.nodes {
            // ...
        }
        unimplemented!()
    }
}
